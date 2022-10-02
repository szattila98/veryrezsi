use entity::recurrence_type::{self, Entity as RecurrenceType};
use sea_orm::{DatabaseConnection, EntityTrait};

use super::error::RecurrenceError;

pub async fn find_recurrence_types(
    conn: &DatabaseConnection,
) -> Result<Vec<recurrence_type::Model>, RecurrenceError> {
    let currency_types = RecurrenceType::find().all(conn).await?;
    Ok(currency_types)
}
