use entity::{
    currency_type::{self, Entity as CurrencyType},
    Id,
};

use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn find_currency_types(
    conn: &DatabaseConnection,
) -> Result<Vec<currency_type::Model>, DbErr> {
    CurrencyType::find().all(conn).await
}

pub async fn find_currency_type_by_id(
    conn: &DatabaseConnection,
    id: Id,
) -> Result<Option<currency_type::Model>, DbErr> {
    CurrencyType::find_by_id(id).one(conn).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use migration::DbErr;
    use sea_orm::{DatabaseBackend, MockDatabase};

    const TEST_ID: Id = 1;
    const TEST_STR: &str = "test";

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
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();

        let currency_types = find_currency_types(&conn).await.expect("not ok");
        let empty_vec = find_currency_types(&conn).await.expect("not ok");
        let db_error = find_currency_types(&conn).await.expect_err("not an error");

        assert_eq!(
            currency_types, mock_currency_types,
            "returned currency type vector is different from expected"
        );
        assert!(
            empty_vec.is_empty(),
            "returned currency type vector should have been empty"
        );
        assert_eq!(
            db_error,
            DbErr::Custom(TEST_STR.to_string()),
            "returned db error variant is different from expected"
        );
    }

    #[tokio::test]
    async fn find_currency_type_by_id_all_cases() {
        let mock_currency_type = currency_type::Model {
            id: TEST_ID,
            abbreviation: TEST_STR.to_string(),
            name: TEST_STR.to_string(),
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![mock_currency_type.clone()], vec![]])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();

        let currency_type = find_currency_type_by_id(&conn, TEST_ID)
            .await
            .expect("not ok");
        let none = find_currency_type_by_id(&conn, TEST_ID)
            .await
            .expect("not ok");
        let db_error = find_currency_type_by_id(&conn, TEST_ID)
            .await
            .expect_err("not an error");

        assert_eq!(
            currency_type,
            Some(mock_currency_type),
            "returned currency type is different from expected"
        );
        assert_eq!(
            none, None,
            "none should have been returned, but it was Some"
        );
        assert_eq!(
            db_error,
            DbErr::Custom(TEST_STR.to_string()),
            "returned db error variant is different from expected"
        );
    }
}
