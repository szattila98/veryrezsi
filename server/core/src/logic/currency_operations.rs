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
    use assert2::check;
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

        let (currency_types, empty_vec, db_error) = tokio::join!(
            find_currency_types(&conn),
            find_currency_types(&conn),
            find_currency_types(&conn)
        );

        check!(currency_types == Ok(mock_currency_types));
        check!(empty_vec == Ok(vec![]));
        check!(db_error == Err(DbErr::Custom(TEST_STR.to_string())));
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

        let (currency_type, none, db_error) = tokio::join!(
            find_currency_type_by_id(&conn, TEST_ID),
            find_currency_type_by_id(&conn, TEST_ID),
            find_currency_type_by_id(&conn, TEST_ID)
        );

        check!(currency_type == Ok(Some(mock_currency_type)));
        check!(none == Ok(None));
        check!(db_error == Err(DbErr::Custom(TEST_STR.to_string())));
    }
}
