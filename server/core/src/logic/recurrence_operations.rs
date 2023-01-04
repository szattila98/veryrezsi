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
    use assert2::check;
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

        let (recurrence_types, empty_vec, db_error) = tokio::join!(
            find_recurrence_types(&conn),
            find_recurrence_types(&conn),
            find_recurrence_types(&conn)
        );

        check!(recurrence_types == Ok(mock_recurrence_types));
        check!(empty_vec == Ok(vec![]));
        check!(db_error == Err(DbErr::Custom(TEST_STR.to_string())));
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

        let (recurrence_type, none, db_error) = tokio::join!(
            find_recurrence_type_by_id(&conn, TEST_ID),
            find_recurrence_type_by_id(&conn, TEST_ID),
            find_recurrence_type_by_id(&conn, TEST_ID)
        );

        check!(recurrence_type == Ok(Some(mock_recurrence_type)));
        check!(none == Ok(None));
        check!(db_error == Err(DbErr::Custom(TEST_STR.to_string())));
    }
}
