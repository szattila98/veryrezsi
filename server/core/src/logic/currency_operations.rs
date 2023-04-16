use entity::currency_type::{self, Entity as CurrencyType};

use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn find_currency_types(
    conn: &DatabaseConnection,
) -> Result<Vec<currency_type::Model>, DbErr> {
    CurrencyType::find().all(conn).await
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
    async fn find_currency_types_all_cases() {
        let mock_currency_types = vec![
            currency_type::Model {
                id: TEST_ID,
                abbreviation: TEST_STR.to_string(),
                name: TEST_STR.to_string(),
            },
            currency_type::Model {
                id: TEST_ID,
                abbreviation: TEST_STR.to_string(),
                name: TEST_STR.to_string(),
            },
            currency_type::Model {
                id: TEST_ID,
                abbreviation: TEST_STR.to_string(),
                name: TEST_STR.to_string(),
            },
        ];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![mock_currency_types.clone(), vec![]])
            .append_query_errors(vec![test_db_error()])
            .into_connection();

        let (currency_types, empty_vec, db_error) = tokio::join!(
            find_currency_types(&conn),
            find_currency_types(&conn),
            find_currency_types(&conn)
        );

        check!(currency_types == Ok(mock_currency_types));
        check!(empty_vec == Ok(vec![]));
        check!(db_error == Err(test_db_error()));
    }
}
