use entity::currency::Entity as Currency;

use migration::DbErr;
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::dto::currencies::CurrencyResponse;

pub async fn find_currencies(conn: &DatabaseConnection) -> Result<Vec<CurrencyResponse>, DbErr> {
    let currencies = Currency::find()
        .all(conn)
        .await?
        .into_iter()
        .map(|currency| currency.into())
        .collect();
    Ok(currencies)
}

#[cfg(test)]
mod tests {
    use crate::logic::common::tests::{TEST_DB_ERROR, TEST_CURRENCY};

    use super::*;
    use assert2::check;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[tokio::test]
    async fn find_currencies_all_cases() {
        let expected_currencies: Vec<CurrencyResponse> = vec![TEST_CURRENCY().into(), TEST_CURRENCY().into()];

        let currencies_stub = vec![TEST_CURRENCY(), TEST_CURRENCY()];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![currencies_stub.clone(), vec![]])
            .append_query_errors(vec![TEST_DB_ERROR()])
            .into_connection();

        let (currencies, empty_vec, db_error) = tokio::join!(
            find_currencies(&conn),
            find_currencies(&conn),
            find_currencies(&conn)
        );

        check!(currencies == Ok(expected_currencies));
        check!(empty_vec == Ok(vec![]));
        check!(db_error == Err(TEST_DB_ERROR()));
    }
}
