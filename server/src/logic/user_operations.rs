use super::error::UserError;
use crate::config;
use crate::email::{render_template, send_mail, ACTIVATION_EMAIL_TEMPLATE};
use crate::routes::dto::NewUserRequest;
use chrono::Duration;
use entity::account_activation::{self, Entity as AccountActivation};
use entity::user::{self, Entity as User};
use entity::Id;
use lettre::AsyncTransport;
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

/// Finds a user by its email in the database.
pub async fn find_user_by_email(
    conn: &DatabaseConnection,
    email: String,
) -> Result<user::Model, UserError> {
    let opt = User::find()
        .filter(user::Column::Email.eq(email.clone()))
        .one(conn)
        .await?;
    match opt {
        Some(user) => Ok(user),
        None => Err(UserError::UserNotFound(email)),
    }
}

/// Finds a user by its id in the database.
pub async fn find_user_by_id(conn: &DatabaseConnection, id: Id) -> Result<user::Model, UserError> {
    match User::find_by_id(id).one(conn).await? {
        Some(user) => Ok(user),
        None => Err(UserError::UserNotFound(id.to_string())),
    }
}

/// Saves a new user to the database, encrypts the password and sends an activation email.
pub async fn save_user<M>(
    config: &config::AppConfig,
    conn: &DatabaseConnection,
    mail_transport: Arc<M>,
    req: NewUserRequest,
) -> Result<user::Model, UserError>
where
    M: AsyncTransport + Send + Sync + 'static,
    <M as AsyncTransport>::Error: std::fmt::Debug,
{
    match User::find()
        .filter(user::Column::Email.eq(req.email.clone()))
        .one(conn)
        .await?
    {
        Some(_) => Err(UserError::EmailAlreadyExists(req.email)),
        None => {
            let pw_hash = bcrypt::hash(req.password)?;

            let (user, activation) = conn
                .transaction::<_, (user::Model, account_activation::Model), UserError>(|txn| {
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
                                .unwrap()),
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

/// Activates a newly registered user.
pub async fn activate_account(conn: &DatabaseConnection, token: String) -> Result<(), UserError> {
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
                        conn.transaction::<_, (), UserError>(|txn| {
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
                    None => Err(UserError::UserNotFound(activation.user_id.to_string())),
                }
            } else {
                Err(UserError::ActivationTokenExpired)
            }
        }
        None => Err(UserError::ActivationTokenNotFound(token)),
    }
}

/**
 * Utility method to authorize if a user should be able to access a resouce.
 * Checks the equality of two user_ids.
 */
pub fn authorize_user_by_id(user_id: Id, user_id_in_resource: Id) -> Result<(), UserError> {
    if user_id != user_id_in_resource {
        return Err(UserError::UserHasNoRightForAction);
    }
    Ok(())
}
