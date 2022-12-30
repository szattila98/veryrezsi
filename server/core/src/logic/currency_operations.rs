use super::error::CurrencyError;

use entity::currency_type::{self, Entity as CurrencyType};

use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn find_currency_types(
    conn: &DatabaseConnection,
) -> Result<Vec<currency_type::Model>, CurrencyError> {
    Ok(CurrencyType::find().all(conn).await?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use migration::DbErr;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[tokio::test]
    async fn find_currency_types_returns_all() {
        let mock_currency_types = vec![
            currency_type::Model {
                id: 1,
                abbreviation: "Helen Love".to_string(),
                name: "Algeria".to_string(),
            },
            currency_type::Model {
                id: 2,
                abbreviation: "Emma Martinez".to_string(),
                name: "Honduras".to_string(),
            },
            currency_type::Model {
                id: 3,
                abbreviation: "Gertrude Baker".to_string(),
                name: "Palau".to_string(),
            },
        ];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![mock_currency_types.clone()])
            .into_connection();

        let currency_types = find_currency_types(&conn).await.unwrap();

        assert_eq!(currency_types, mock_currency_types);
    }

    #[tokio::test]
    #[should_panic]
    async fn find_currency_types_returns_an_error() {
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_errors(vec![DbErr::Custom("Mozsojuc".to_string())])
            .into_connection();
        find_currency_types(&conn).await.unwrap();
    }
}
