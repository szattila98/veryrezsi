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
use tracing::debug;

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
    match find_user_by_email(conn, req.email.clone()).await? {
        Some(_) => Err(SaveUserError::UserAlreadyExists),
        None => {
            let pw_hash = match bcrypt::hash(req.password) {
                Ok(hashed) => hashed,
                Err(error) => {
                    return Err(SaveUserError::PasswordCannotBeHashed(format!("{error}")))
                }
            };
            let (user, activation) = conn
                .transaction::<_, (user::Model, account_activation::Model), SaveUserError>(|txn| {
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

                        Ok((user, activation))
                    })
                })
                .await?;

            let activation_link = format!(
                "http://{}/api/user/activate/{}",
                config.server_address, activation.token
            );
            let mut data = HashMap::new();
            data.insert("username", &user.username);
            data.insert("activation_link", &activation_link);
            let body = render_template(ACTIVATION_EMAIL_TEMPLATE, data);
            let email = user.email.clone();
            tokio::spawn(async move {
                send_mail(mail_transport, email, "Veryrezsi account activation", body).await;
                debug!("Sent activation email");
            });

            Ok(user)
        }
    }
}

pub async fn activate_account(
    conn: &DatabaseConnection,
    token: String,
) -> Result<(), ActivateAccountError> {
    let account_activation = AccountActivation::find()
        .filter(account_activation::Column::Token.eq(token.clone()))
        .one(conn)
        .await?;
    match account_activation {
        Some(activation) => {
            if activation.expiration >= chrono::Local::now() {
                let user = User::find_by_id(activation.user_id).one(conn).await?;
                match user {
                    Some(user) => {
                        conn.transaction::<_, (), ActivateAccountError>(|txn| {
                            Box::pin(async move {
                                let mut user = user.into_active_model();
                                user.activated = Set(true);
                                user.update(txn).await?;
                                let activation = activation.into_active_model();
                                activation.delete(txn).await?;
                                Ok(())
                            })
                        })
                        .await?;
                        Ok(())
                    }
                    None => Err(ActivateAccountError::InvalidToken),
                }
            } else {
                Err(ActivateAccountError::InvalidToken)
            }
        }
        None => Err(ActivateAccountError::InvalidToken),
    }
}

/// Utility method to authorize if a user should be able to access a resource.
/// Checks the equality of two user_ids.
///
/// # Errors
///
/// This function will return an error if the two ids are not equal.
pub fn authorize_user_by_id(
    user_id: Id,
    user_id_in_resource: Id,
) -> Result<(), AuthorizeUserError> {
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

    #[test]
    fn find_by_email_all_cases() {}

    #[test]
    fn find_by_id_all_cases() {}

    #[test]
    fn save_user_all_cases() {}

    #[test]
    fn activate_account_all_cases() {}

    #[test]
    fn authorize_user_by_id_all_cases() {}
}
