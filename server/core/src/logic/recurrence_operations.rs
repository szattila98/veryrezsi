use entity::{
    recurrence_type::{self, Entity as RecurrenceType},
    Id,
};

use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn find_recurrence_types(
    conn: &DatabaseConnection,
) -> Result<Vec<recurrence_type::Model>, DbErr> {
    RecurrenceType::find().all(conn).await
}

pub async fn find_recurrence_type_by_id(
    conn: &DatabaseConnection,
    id: Id,
) -> Result<Option<recurrence_type::Model>, DbErr> {
    RecurrenceType::find_by_id(id).one(conn).await
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use migration::DbErr;
    use sea_orm::{DatabaseBackend, MockDatabase};

    const TEST_ID: Id = 1;
    const TEST_FLOAT: f64 = 1.0;
    const TEST_STR: &str = "test";

    #[tokio::test]
    async fn find_recurrence_types_all_cases() {
        let mock_recurrence_types = vec![
            recurrence_type::Model {
                id: TEST_ID,
                name: TEST_STR.to_string(),
                per_year: TEST_FLOAT,
            },
            recurrence_type::Model {
                id: TEST_ID,
                name: TEST_STR.to_string(),
                per_year: TEST_FLOAT,
            },
            recurrence_type::Model {
                id: TEST_ID,
                name: TEST_STR.to_string(),
                per_year: TEST_FLOAT,
            },
        ];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![mock_recurrence_types.clone(), vec![]])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();

        let recurrence_types = find_recurrence_types(&conn).await.expect("not ok");
        let empty_vec = find_recurrence_types(&conn).await.expect("not ok");
        let db_error = find_recurrence_types(&conn)
            .await
            .expect_err("not an error");

        assert_eq!(
            recurrence_types, mock_recurrence_types,
            "returned recurrence type vector is different from expected"
        );
        assert!(
            empty_vec.is_empty(),
            "returned recurrence type vector should have been empty"
        );
        assert_eq!(
            db_error,
            DbErr::Custom(TEST_STR.to_string()),
            "returned db error variant is different from expected"
        );
    }

    #[tokio::test]
    async fn find_recurrence_type_by_id_all_cases() {
        let mock_recurrence_type = recurrence_type::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            per_year: TEST_FLOAT,
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![mock_recurrence_type.clone()], vec![]])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();

        let recurrence_type = find_recurrence_type_by_id(&conn, TEST_ID)
            .await
            .expect("not ok");
        let none = find_recurrence_type_by_id(&conn, TEST_ID)
            .await
            .expect("not ok");
        let db_error = find_recurrence_type_by_id(&conn, TEST_ID)
            .await
            .expect_err("not an error");

        assert_eq!(
            recurrence_type,
            Some(mock_recurrence_type),
            "returned recurrence type is different from expected"
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
