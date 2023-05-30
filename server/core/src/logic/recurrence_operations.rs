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

    use crate::logic::common::tests::{
        test_db_error,
        test_recurrence,
    };

    use super::*;
    use assert2::check;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[tokio::test]
    async fn find_recurrences_all_cases() {
        let expected_recurrences: Vec<RecurrenceResponse> = vec![test_recurrence().into(), test_recurrence().into()];

        let recurrences_stub = vec![test_recurrence(), test_recurrence()];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![recurrences_stub.clone(), vec![]])
            .append_query_errors(vec![test_db_error()])
            .into_connection();

        let (recurrences, empty_vec, db_error) = tokio::join!(
            find_recurrences(&conn),
            find_recurrences(&conn),
            find_recurrences(&conn)
        );

        check!(recurrences == Ok(expected_recurrences));
        check!(empty_vec == Ok(vec![]));
        check!(db_error == Err(test_db_error()));
    }
}
