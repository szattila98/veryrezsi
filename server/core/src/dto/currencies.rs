use entity::{currencies, Id};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct CurrencyResponse {
    pub id: Id,
    pub abbreviation: String,
    pub name: String,
}

impl From<currencies::Model> for CurrencyResponse {
    fn from(currency: currencies::Model) -> Self {
        Self {
            id: currency.id,
            abbreviation: currency.abbreviation,
            name: currency.name,
        }
    }
}
