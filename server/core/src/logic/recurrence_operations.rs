use entity::recurrence::Entity as Recurrence;

use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::dto::recurrences::RecurrenceResponse;

pub async fn find_recurrences(conn: &DatabaseConnection) -> Result<Vec<RecurrenceResponse>, DbErr> {
    let recurrences = Recurrence::find()
        .all(conn)
        .await?
        .into_iter()
        .map(|recurrence| recurrence.into())
        .collect();
    Ok(recurrences)
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
    async fn find_recurrences_all_cases() {
        let mock_recurrences = vec![
            recurrence::Model {
                id: TEST_ID,
                name: TEST_STR.to_string(),
                per_year: TEST_FLOAT,
            },
            recurrence::Model {
                id: TEST_ID,
                name: TEST_STR.to_string(),
                per_year: TEST_FLOAT,
            },
            recurrence::Model {
                id: TEST_ID,
                name: TEST_STR.to_string(),
                per_year: TEST_FLOAT,
            },
        ];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![mock_recurrences.clone(), vec![]])
            .append_query_errors(vec![test_db_error()])
            .into_connection();

        let (recurrences, empty_vec, db_error) = tokio::join!(
            find_recurrences(&conn),
            find_recurrences(&conn),
            find_recurrences(&conn)
        );

        check!(recurrences == Ok(mock_recurrences));
        check!(empty_vec == Ok(vec![]));
        check!(db_error == Err(test_db_error()));
    }
}
