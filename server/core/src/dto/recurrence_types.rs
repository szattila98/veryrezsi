use entity::{recurrence_type, Id};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct RecurrenceTypeResponse {
    pub id: Id,
    pub name: String,
    pub per_year: f64,
}

impl From<recurrence_type::Model> for RecurrenceTypeResponse {
    fn from(recurrence_type: recurrence_type::Model) -> Self {
        Self {
            id: recurrence_type.id,
            name: recurrence_type.name,
            per_year: recurrence_type.per_year,
        }
    }
}
