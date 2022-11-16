use super::error::CurrencyError;

use entity::currency_type::{self, Entity as CurrencyType};

use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn find_currency_types(
    conn: &DatabaseConnection,
) -> Result<Vec<currency_type::Model>, CurrencyError> {
    let currency_types = CurrencyType::find().all(conn).await?;
    Ok(currency_types)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::MockDatabase;

    #[test]
    fn fun() {
        assert_eq!(1, 1);
    }
}
