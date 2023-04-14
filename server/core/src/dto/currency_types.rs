use entity::{currency_type, Id};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct CurrencyTypeResponse {
    pub id: Id,
    pub abbreviation: String,
    pub name: String,
}

impl From<currency_type::Model> for CurrencyTypeResponse {
    fn from(currency_type: currency_type::Model) -> Self {
        Self {
            id: currency_type.id,
            abbreviation: currency_type.abbreviation,
            name: currency_type.name,
        }
    }
}
