use self::errors::{ActivateAccountError, AuthorizeUserError, SaveUserError};

use crate::config;
use crate::dto::users::NewUserRequest;
use crate::email::{render_template, send_mail, ACTIVATION_EMAIL_TEMPLATE};
use chrono::Duration;
use entity::account_activation::{self, Entity as AccountActivation};
use entity::user::{self, Entity as User};
use entity::Id;
use lettre::AsyncTransport;
use migration::DbErr;
use pwhash::bcrypt;
use sea_orm::prelude::Uuid;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set, TransactionTrait,
};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn find_user_by_email(
    conn: &DatabaseConnection,
    email: String,
) -> Result<Option<user::Model>, DbErr> {
    let user = User::find()
        .filter(user::Column::Email.eq(email.clone()))
        .one(conn)
        .await?;
    Ok(user)
}

pub async fn find_user_by_id(
    conn: &DatabaseConnection,
    id: Id,
) -> Result<Option<user::Model>, DbErr> {
    let user = User::find_by_id(id).one(conn).await?;
    Ok(user)
}

pub async fn save_user<M>(
    config: &config::AppConfig,
    conn: &DatabaseConnection,
    mail_transport: Arc<M>,
    req: NewUserRequest,
) -> Result<user::Model, SaveUserError>
where
    M: AsyncTransport + Send + Sync + 'static,
    <M as AsyncTransport>::Error: std::fmt::Debug,
{
    let None = find_user_by_email(conn, req.email.clone()).await? else {
        return Err(SaveUserError::UserAlreadyExists)
    };

    let pw_hash = match bcrypt::hash(req.password) {
        Ok(hashed) => hashed,
        Err(error) => return Err(SaveUserError::PasswordCannotBeHashed(format!("{error}"))),
    };

    let server_address = config.server_address;
    let user = conn
        .transaction::<_, user::Model, SaveUserError>(|txn| {
            Box::pin(async move {
                let user = user::ActiveModel {
                    id: NotSet,
                    email: Set(req.email),
                    username: Set(req.username),
                    pw_hash: Set(pw_hash),
                    activated: NotSet,
                };
                let user = user.insert(txn).await?;

                let activation = account_activation::ActiveModel {
                    id: NotSet,
                    token: Set(Uuid::new_v4().to_string()),
                    user_id: Set(user.id),
                    expiration: Set(chrono::Local::now()
                        .checked_add_signed(Duration::days(1))
                        .expect("we should not be this far ahead into the future Marty, the date overflowed the bounds")),
                };
                let activation = activation.insert(txn).await?;

                let activation_link = format!(
                    "http://{}/api/user/activate/{}",
                    server_address, activation.token
                );
                let mut data = HashMap::new();
                data.insert("username", &user.username);
                data.insert("activation_link", &activation_link);
                let body = render_template(ACTIVATION_EMAIL_TEMPLATE, data);
                let email = user.email.clone();
                match send_mail(mail_transport, email, "Veryrezsi account activation", body).await {
                    Ok(_) => Ok(user),
                    Err(reason) => Err(SaveUserError::EmailCannotBeSent(reason)),
                }
            })
        })
        .await?;
    Ok(user)
}

pub async fn activate_account(
    conn: &DatabaseConnection,
    token: String,
) -> Result<(), ActivateAccountError> {
    let Some(account_activation) = AccountActivation::find()
        .filter(account_activation::Column::Token.eq(token.clone()))
        .one(conn)
        .await? else {
        return Err(ActivateAccountError::InvalidToken);
    };

    if account_activation.expiration < chrono::Local::now() {
        return Err(ActivateAccountError::InvalidToken);
    }

    let Some(user) = User::find_by_id(account_activation.user_id)
        .one(conn)
        .await? else {
        return Err(ActivateAccountError::InvalidToken);
    };

    conn.transaction::<_, (), ActivateAccountError>(|txn| {
        Box::pin(async move {
            let mut user = user.into_active_model();
            user.activated = Set(true);
            user.update(txn).await?;
            let activation = account_activation.into_active_model();
            activation.delete(txn).await?;
            Ok(())
        })
    })
    .await?;
    Ok(())
}

/// Utility method to authorize if a user should be able to access a resource.
/// Checks the equality of two user_ids.
///
/// # Errors
///
/// This function will return an error if the two ids are not equal.
pub fn authorize_user(user_id: Id, user_id_in_resource: Id) -> Result<(), AuthorizeUserError> {
    if user_id != user_id_in_resource {
        return Err(AuthorizeUserError);
    }
    Ok(())
}

pub mod errors {
    use migration::DbErr;
    use sea_orm::TransactionError;
    use thiserror::Error;

    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum SaveUserError {
        #[error("user already exists")]
        UserAlreadyExists,
        #[error("{0}")]
        PasswordCannotBeHashed(String),
        #[error("{0}")]
        EmailCannotBeSent(String),
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }

    impl From<TransactionError<SaveUserError>> for SaveUserError {
        fn from(e: TransactionError<SaveUserError>) -> Self {
            match e {
                TransactionError::Connection(e) => e.into(),
                TransactionError::Transaction(e) => e,
            }
        }
    }

    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum ActivateAccountError {
        #[error("invalid token")]
        InvalidToken,
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }

    impl From<TransactionError<ActivateAccountError>> for ActivateAccountError {
        fn from(e: TransactionError<ActivateAccountError>) -> Self {
            match e {
                TransactionError::Connection(e) => e.into(),
                TransactionError::Transaction(e) => e,
            }
        }
    }

    #[derive(Error, Debug, PartialEq, Eq)]
    #[error("user is not authorized")]
    pub struct AuthorizeUserError;
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;

    use assert2::check;
    use chrono::Duration;
    use entity::{account_activation, user};
    use lettre::transport::stub::AsyncStubTransport;
    use migration::DbErr;
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

    use crate::{
        config::{AppConfig, MailConfig},
        dto::users::NewUserRequest,
        logic::user_operations::{
            activate_account, authorize_user,
            errors::{ActivateAccountError, AuthorizeUserError, SaveUserError},
            find_user_by_email, find_user_by_id, save_user,
        },
    };

    const TEST_STR: &str = "test";
    const TEST_EMAIL: &str = "test@test.com";
    const TEST_ID: u64 = 1;

    #[tokio::test]
    async fn find_by_email_all_cases() {
        let mock_user = user::Model {
            id: TEST_ID,
            email: TEST_EMAIL.to_string(),
            username: TEST_STR.to_string(),
            pw_hash: TEST_STR.to_string(),
            activated: true,
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![mock_user.clone()], vec![]])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();

        let (user, not_found, db_error) = tokio::join!(
            find_user_by_email(&conn, TEST_EMAIL.to_string()),
            find_user_by_email(&conn, TEST_EMAIL.to_string()),
            find_user_by_email(&conn, TEST_EMAIL.to_string())
        );

        check!(user == Ok(Some(mock_user)));
        check!(not_found == Ok(None));
        check!(db_error == Err(DbErr::Custom(TEST_STR.to_string())));
    }

    #[tokio::test]
    async fn find_by_id_all_cases() {
        let mock_user = user::Model {
            id: TEST_ID,
            email: TEST_EMAIL.to_string(),
            username: TEST_STR.to_string(),
            pw_hash: TEST_STR.to_string(),
            activated: true,
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![mock_user.clone()], vec![]])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();

        let (user, not_found, db_error) = tokio::join!(
            find_user_by_id(&conn, TEST_ID),
            find_user_by_id(&conn, TEST_ID),
            find_user_by_id(&conn, TEST_ID)
        );

        check!(user == Ok(Some(mock_user)));
        check!(not_found == Ok(None));
        check!(db_error == Err(DbErr::Custom(TEST_STR.to_string())));
    }

    #[tokio::test]
    async fn save_user_all_cases() {
        let mock_user = user::Model {
            id: TEST_ID,
            email: TEST_EMAIL.to_string(),
            username: TEST_STR.to_string(),
            pw_hash: TEST_STR.to_string(),
            activated: true,
        };
        let mock_account_activation = account_activation::Model {
            id: TEST_ID,
            user_id: TEST_ID,
            expiration: chrono::Local::now(),
            token: TEST_STR.to_string(),
        };
        let config = AppConfig {
            server_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080),
            database_url: TEST_STR.to_string(),
            cookie_key: TEST_STR.to_string(),
            log_level: crate::config::LogLevel::Trace,
            mail_config: MailConfig {
                smtp_address: TEST_STR.to_string(),
                smtp_port: 7777,
                smtp_username: TEST_STR.to_string(),
                smtp_password: TEST_STR.to_string(),
            },
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // happy case
            .append_query_results(vec![vec![], vec![mock_user.clone()]])
            .append_exec_results(vec![MockExecResult::default()])
            .append_exec_results(vec![MockExecResult::default()])
            .append_query_results(vec![vec![mock_account_activation.clone()]])
            // user already exists error
            .append_query_results(vec![vec![mock_user.clone()]])
            // password error cannot be tested as it only happens if system random number generator cannot be opened
            // email_error
            .append_query_results(vec![vec![], vec![mock_user.clone()]])
            .append_exec_results(vec![MockExecResult::default()])
            .append_exec_results(vec![MockExecResult::default()])
            .append_query_results(vec![vec![mock_account_activation]])
            // db_error - on user by email query
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            // db_error - on user insert
            .append_query_results(vec![Vec::<user::Model>::new()])
            .append_exec_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            // db_error - on account activation insert
            .append_query_results(vec![vec![], vec![mock_user.clone()]])
            .append_exec_results(vec![MockExecResult::default()])
            .append_exec_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();
        let ok_mail_transport = Arc::new(AsyncStubTransport::new_ok());
        let error_mail_transport = Arc::new(AsyncStubTransport::new_error());
        let request = NewUserRequest {
            email: TEST_EMAIL.to_string(),
            username: TEST_STR.to_string(),
            password: TEST_STR.to_string(),
            confirm_password: TEST_STR.to_string(),
        };

        let (
            user_saved,
            user_alredy_exists_error,
            email_error,
            user_email_db_error,
            user_insert_db_error,
            activation_insert_db_error,
        ) = tokio::join!(
            save_user(&config, &conn, ok_mail_transport.clone(), request.clone()),
            save_user(&config, &conn, ok_mail_transport.clone(), request.clone()),
            save_user(&config, &conn, error_mail_transport, request.clone()),
            save_user(&config, &conn, ok_mail_transport.clone(), request.clone()),
            save_user(&config, &conn, ok_mail_transport.clone(), request.clone()),
            save_user(&config, &conn, ok_mail_transport, request),
        );

        let db_error = Err(SaveUserError::DatabaseError(DbErr::Custom(
            TEST_STR.to_string(),
        )));
        check!(user_saved == Ok(mock_user));
        check!(user_alredy_exists_error == Err(SaveUserError::UserAlreadyExists));
        check!(email_error == Err(SaveUserError::EmailCannotBeSent("Error".to_string())));
        check!(user_email_db_error == db_error);
        check!(user_insert_db_error == db_error);
        check!(activation_insert_db_error == db_error);
    }

    #[tokio::test]
    async fn activate_account_all_cases() {
        let mock_account_activation = account_activation::Model {
            id: TEST_ID,
            user_id: TEST_ID,
            expiration: chrono::Local::now()
                .checked_add_signed(Duration::days(15))
                .unwrap(),
            token: TEST_STR.to_string(),
        };
        let expired_activation = account_activation::Model {
            id: TEST_ID,
            user_id: TEST_ID,
            expiration: chrono::Local::now()
                .checked_sub_signed(Duration::days(15))
                .unwrap(),
            token: TEST_STR.to_string(),
        };
        let mock_user = user::Model {
            id: TEST_ID,
            email: TEST_EMAIL.to_string(),
            username: TEST_STR.to_string(),
            pw_hash: TEST_STR.to_string(),
            activated: true,
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // happy case
            .append_query_results(vec![vec![mock_account_activation.clone()]])
            .append_query_results(vec![vec![mock_user.clone()], vec![mock_user.clone()]])
            .append_exec_results(vec![
                MockExecResult {
                    last_insert_id: TEST_ID,
                    rows_affected: 1,
                },
                MockExecResult::default(),
            ])
            // account_activation not found
            .append_query_results(vec![Vec::<account_activation::Model>::new()])
            // user not found
            .append_query_results(vec![vec![mock_account_activation.clone()]])
            .append_query_results(vec![Vec::<user::Model>::new()])
            // expired activation
            .append_query_results(vec![vec![expired_activation]])
            // db error - account activation query failed
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            // db error - user query failed
            .append_query_results(vec![vec![mock_account_activation.clone()]])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            // db error - user update failed
            .append_query_results(vec![vec![mock_account_activation.clone()]])
            .append_query_results(vec![vec![mock_user.clone()], vec![mock_user.clone()]])
            .append_exec_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();

        let (
            happy_path,
            account_activation_not_found,
            user_not_found,
            expired_activation,
            activation_query_db_error,
            user_query_db_error,
            user_update_db_error,
        ) = tokio::join!(
            activate_account(&conn, TEST_STR.to_string()),
            activate_account(&conn, TEST_STR.to_string()),
            activate_account(&conn, TEST_STR.to_string()),
            activate_account(&conn, TEST_STR.to_string()),
            activate_account(&conn, TEST_STR.to_string()),
            activate_account(&conn, TEST_STR.to_string()),
            activate_account(&conn, TEST_STR.to_string()),
        );

        let invalid_token_err = Err(ActivateAccountError::InvalidToken);
        let db_error = Err(ActivateAccountError::DatabaseError(DbErr::Custom(
            TEST_STR.to_string(),
        )));
        check!(happy_path == Ok(()));
        check!(account_activation_not_found == invalid_token_err);
        check!(user_not_found == invalid_token_err);
        check!(expired_activation == invalid_token_err);
        check!(activation_query_db_error == db_error);
        check!(user_query_db_error == db_error);
        check!(user_update_db_error == db_error);
    }

    #[test]
    fn authorize_user_by_id_all_cases() {
        let ok = authorize_user(TEST_ID, TEST_ID);
        let error = authorize_user(TEST_ID, TEST_ID - 1);

        check!(ok == Ok(()));
        check!(error == Err(AuthorizeUserError));
    }
}
