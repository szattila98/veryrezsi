use entity::{currency, Id};
use serde::Serialize;

#[derive(Clone, Serialize, PartialEq, Eq)]
pub struct CurrencyResponse {
    pub id: Id,
    pub abbreviation: String,
    pub name: String,
}

impl From<currency::Model> for CurrencyResponse {
    fn from(currency: currency::Model) -> Self {
        Self {
            id: currency.id,
            abbreviation: currency.abbreviation,
            name: currency.name,
        }
    }
}
