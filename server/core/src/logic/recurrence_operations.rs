use entity::recurrence_type::{self, Entity as RecurrenceType};

use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn find_recurrence_types(
    conn: &DatabaseConnection,
) -> Result<Vec<recurrence_type::Model>, DbErr> {
    RecurrenceType::find().all(conn).await
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use assert2::check;
    use entity::Id;
    use migration::DbErr;
    use sea_orm::{DatabaseBackend, MockDatabase};

    const TEST_ID: Id = 1;
    const TEST_FLOAT: f64 = 1.0;
    const TEST_STR: &str = "test";

    fn test_db_error() -> DbErr {
        DbErr::Custom(TEST_STR.to_string())
    }

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
            .append_query_errors(vec![test_db_error()])
            .into_connection();

        let (recurrence_types, empty_vec, db_error) = tokio::join!(
            find_recurrence_types(&conn),
            find_recurrence_types(&conn),
            find_recurrence_types(&conn)
        );

        check!(recurrence_types == Ok(mock_recurrence_types));
        check!(empty_vec == Ok(vec![]));
        check!(db_error == Err(test_db_error()));
    }
}
