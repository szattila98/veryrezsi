use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait, PrimaryKeyTrait};

pub static DATE_FORMAT: &str = "%d-%m-%Y";

pub async fn find_entity_by_id<E: EntityTrait>(
    conn: &DatabaseConnection,
    id: <<E as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
) -> Result<Option<E::Model>, DbErr> {
    E::find_by_id(id).one(conn).await
}

#[cfg(test)]
pub mod tests {
    use assert2::check;
    use chrono::{Duration, NaiveDate};
    use entity::{
        account_activation, currency, expense, predefined_expense, recurrence, transaction, user,
        Id,
    };
    use migration::DbErr;
    use sea_orm::entity::prelude::*;
    use sea_orm::{DatabaseBackend, DeriveActiveModelBehavior, DeriveEntityModel, MockDatabase};
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    use crate::{
        config::{AppConfig, MailConfig},
        logic::find_entity_by_id,
    };

    pub const TEST_STR: &str = "test";
    pub const TEST_EMAIL: &str = "test@test.com";
    pub const TEST_ID: u64 = 1;
    pub const TEST_FLOAT: f64 = 1.0;
    pub const TEST_DATE: &str = "06-08-1998";

    pub fn test_db_error() -> DbErr {
        DbErr::Custom(TEST_STR.to_string())
    }

    pub fn test_currency() -> currency::Model {
        return currency::Model {
            id: TEST_ID,
            abbreviation: TEST_STR.to_string(),
            name: TEST_STR.to_string(),
        };
    }

    pub fn test_recurrence() -> recurrence::Model {
        return recurrence::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            per_year: TEST_FLOAT,
        };
    }

    pub fn test_expense() -> expense::Model {
        return expense::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: test_decimal(),
            start_date: NaiveDate::MIN,
            user_id: TEST_ID,
            currency_id: TEST_ID,
            recurrence_id: TEST_ID,
            predefined_expense_id: Some(TEST_ID),
        };
    }

    pub fn test_predefined_expense() -> predefined_expense::Model {
        return predefined_expense::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: test_decimal(),
            currency_id: TEST_ID,
            recurrence_id: TEST_ID,
        };
    }

    pub fn test_transaction() -> transaction::Model {
        return transaction::Model {
            id: TEST_ID,
            donor_name: TEST_STR.to_string(),
            value: test_decimal(),
            date: NaiveDate::MIN,
            currency_id: TEST_ID,
            expense_id: TEST_ID,
        };
    }

    pub fn test_transaction_2() -> transaction::Model {
        return transaction::Model {
            id: test_transaction().id + 1,
            ..test_transaction()
        };
    }

    pub fn test_user() -> user::Model {
        return user::Model {
            id: TEST_ID,
            email: TEST_EMAIL.to_string(),
            username: TEST_STR.to_string(),
            pw_hash: TEST_STR.to_string(),
            activated: true,
        };
    }

    pub fn test_account_activation() -> account_activation::Model {
        return account_activation::Model {
            id: TEST_ID,
            user_id: TEST_ID,
            expiration: chrono::Local::now()
                .checked_add_signed(Duration::days(1))
                .unwrap(),
            token: TEST_STR.to_string(),
        };
    }

    pub fn test_app_config() -> AppConfig {
        return AppConfig {
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
    }

    pub fn test_decimal() -> Decimal {
        Decimal::new(1, 2)
    }

    #[derive(Clone, Debug, PartialEq, Eq, DeriveActiveModelBehavior, DeriveEntityModel)]
    #[sea_orm(table_name = "test")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Id,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    #[tokio::test]
    async fn find_entity_by_id_all_cases() {
        let mock_model = Model { id: TEST_ID };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![mock_model.clone()], vec![]])
            .append_query_errors(vec![test_db_error()])
            .into_connection();

        let (model, not_found, db_error) = tokio::join!(
            find_entity_by_id::<Entity>(&conn, TEST_ID),
            find_entity_by_id::<Entity>(&conn, TEST_ID),
            find_entity_by_id::<Entity>(&conn, TEST_ID)
        );

        check!(model == Ok(Some(mock_model)));
        check!(not_found == Ok(None));
        check!(db_error == Err(test_db_error()));
    }
}
