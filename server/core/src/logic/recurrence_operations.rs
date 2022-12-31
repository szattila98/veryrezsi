use entity::recurrence_type::{self, Entity as RecurrenceType};

use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn find_recurrence_types(
    conn: &DatabaseConnection,
) -> Result<Vec<recurrence_type::Model>, DbErr> {
    let currency_types = RecurrenceType::find().all(conn).await?;
    Ok(currency_types)
}

#[cfg(test)]
mod tests {
    use super::*;
    use migration::DbErr;
    use sea_orm::{DatabaseBackend, MockDatabase};

    const TEST_STR: &str = "test";

    #[tokio::test]
    async fn find_currency_types_all_cases() {
        let mock_recurrence_types = vec![
            recurrence_type::Model {
                id: 1,
                name: TEST_STR.to_string(),
                per_year: 1.0,
            },
            recurrence_type::Model {
                id: 1,
                name: TEST_STR.to_string(),
                per_year: 1.0,
            },
            recurrence_type::Model {
                id: 1,
                name: TEST_STR.to_string(),
                per_year: 1.0,
            },
        ];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![mock_recurrence_types.clone()])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();

        let recurrence_types = find_recurrence_types(&conn).await.expect("not ok");
        let db_error = find_recurrence_types(&conn)
            .await
            .expect_err("not an error");

        assert_eq!(
            recurrence_types, mock_recurrence_types,
            "returned recurrence type vector is different from expected"
        );
        assert!(
            db_error == DbErr::Custom(TEST_STR.to_string()),
            "returned db error variant is different from expected"
        );
    }
}
