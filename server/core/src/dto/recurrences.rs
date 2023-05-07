use entity::{recurrences, Id};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct RecurrenceResponse {
    pub id: Id,
    pub name: String,
    pub per_year: f64,
}

impl From<recurrences::Model> for RecurrenceResponse {
    fn from(recurrence: recurrences::Model) -> Self {
        Self {
            id: recurrence.id,
            name: recurrence.name,
            per_year: recurrence.per_year,
        }
    }
}
