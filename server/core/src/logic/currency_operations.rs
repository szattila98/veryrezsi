use entity::currency::{self, Entity as Currency};

use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn find_currencies(conn: &DatabaseConnection) -> Result<Vec<currency::Model>, DbErr> {
    Currency::find().all(conn).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;
    use entity::Id;
    use migration::DbErr;
    use sea_orm::{DatabaseBackend, MockDatabase};

    const TEST_ID: Id = 1;
    const TEST_STR: &str = "test";

    fn test_db_error() -> DbErr {
        DbErr::Custom(TEST_STR.to_string())
    }

    #[tokio::test]
    async fn find_currencies_all_cases() {
        let mock_currencies = vec![
            currency::Model {
                id: TEST_ID,
                abbreviation: TEST_STR.to_string(),
                name: TEST_STR.to_string(),
            },
            currency::Model {
                id: TEST_ID,
                abbreviation: TEST_STR.to_string(),
                name: TEST_STR.to_string(),
            },
            currency::Model {
                id: TEST_ID,
                abbreviation: TEST_STR.to_string(),
                name: TEST_STR.to_string(),
            },
        ];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![mock_currencies.clone(), vec![]])
            .append_query_errors(vec![test_db_error()])
            .into_connection();

        let (currencies, empty_vec, db_error) = tokio::join!(
            find_currencies(&conn),
            find_currencies(&conn),
            find_currencies(&conn)
        );

        check!(currencies == Ok(mock_currencies));
        check!(empty_vec == Ok(vec![]));
        check!(db_error == Err(test_db_error()));
    }
}
