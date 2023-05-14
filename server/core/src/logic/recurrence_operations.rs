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
    use entity::{Id, recurrence};
    use migration::DbErr;
    use sea_orm::{DatabaseBackend, MockDatabase};

    const TEST_ID: Id = 1;
    const TEST_FLOAT: f64 = 1.0;
    const TEST_STR: &str = "test";

    const TEST_RECURRENCE: recurrence::Model = recurrence::Model {
        id: TEST_ID,
        name: TEST_STR.to_string(),
        per_year: TEST_FLOAT,
    };

    const fn TEST_DB_ERROR() -> DbErr {
        DbErr::Custom(TEST_STR.to_string())
    }

    #[tokio::test]
    async fn find_recurrences_all_cases() {
        let expected_recurrences: Vec<RecurrenceResponse> = vec![TEST_RECURRENCE.into(), TEST_RECURRENCE.into()];

        let recurrences_stub = vec![TEST_RECURRENCE, TEST_RECURRENCE, TEST_RECURRENCE];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![recurrences_stub.clone(), vec![]])
            .append_query_errors(vec![TEST_DB_ERROR()])
            .into_connection();

        let (recurrences, empty_vec, db_error) = tokio::join!(
            find_recurrences(&conn),
            find_recurrences(&conn),
            find_recurrences(&conn)
        );

        check!(recurrences == Ok(expected_recurrences));
        check!(empty_vec == Ok(vec![]));
        check!(db_error == Err(TEST_DB_ERROR()));
    }
}
