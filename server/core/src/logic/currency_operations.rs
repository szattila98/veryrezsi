use entity::currency_type::{self, Entity as CurrencyType};

use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn find_currency_types(
    conn: &DatabaseConnection,
) -> Result<Vec<currency_type::Model>, DbErr> {
    Ok(CurrencyType::find().all(conn).await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use migration::DbErr;
    use sea_orm::{DatabaseBackend, MockDatabase};

    const TEST_STR: &str = "test";

    #[tokio::test]
    async fn find_currency_types_all_cases() {
        let mock_currency_types = vec![
            currency_type::Model {
                id: 1,
                abbreviation: TEST_STR.to_string(),
                name: TEST_STR.to_string(),
            },
            currency_type::Model {
                id: 1,
                abbreviation: TEST_STR.to_string(),
                name: TEST_STR.to_string(),
            },
            currency_type::Model {
                id: 1,
                abbreviation: TEST_STR.to_string(),
                name: TEST_STR.to_string(),
            },
        ];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![mock_currency_types.clone()])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();

        let currency_types = find_currency_types(&conn).await.expect("not ok");
        let db_error = find_currency_types(&conn).await.expect_err("not an error");

        assert_eq!(
            currency_types, mock_currency_types,
            "returned currency type vector is different from expected"
        );
        assert!(
            db_error == DbErr::Custom(TEST_STR.to_string()),
            "returned db error variant is different from expected"
        );
    }
}
