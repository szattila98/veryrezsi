use entity::recurrence_type::{self, Entity as RecurrenceType};

use sea_orm::{DatabaseConnection, EntityTrait};

use super::error::RecurrenceError;

pub async fn find_recurrence_types(
    conn: &DatabaseConnection,
) -> Result<Vec<recurrence_type::Model>, RecurrenceError> {
    let currency_types = RecurrenceType::find().all(conn).await?;
    Ok(currency_types)
}

#[cfg(test)]
mod tests {
    use super::*;
    use migration::DbErr;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[tokio::test]
    async fn find_returns_all() {}

    #[tokio::test]
    #[should_panic]
    async fn find_returns_error() {
        panic!()
    }
}
