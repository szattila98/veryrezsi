use entity::{recurrence, Id};
use serde::Serialize;

#[derive(Clone, Serialize, PartialEq)]
pub struct RecurrenceResponse {
    pub id: Id,
    pub name: String,
    pub per_year: f64,
}

impl From<recurrence::Model> for RecurrenceResponse {
    fn from(recurrence: recurrence::Model) -> Self {
        Self {
            id: recurrence.id,
            name: recurrence.name,
            per_year: recurrence.per_year,
        }
    }
}
