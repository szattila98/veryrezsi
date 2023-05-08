use entity::{currency, transaction, Id, MoneyAmount};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::currencies::CurrencyResponse;

#[derive(Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewTransactionRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "donor name must not be empty or longer than 255 characters"
    ))]
    pub donor_name: String,

    pub currency_id: Id,

    pub value: MoneyAmount,

    pub date: String,

    pub expense_id: Id,
}

#[derive(Clone, Serialize, PartialEq, Eq)]
pub struct TransactionResponse {
    pub id: Id,
    pub donor_name: String,
    pub value: MoneyAmount,
    pub date: Date,
    pub currency: CurrencyResponse,
}

pub type TransactionResponseParts = (transaction::Model, currency::Model);
impl From<TransactionResponseParts> for TransactionResponse {
    fn from((transaction, currency): TransactionResponseParts) -> Self {
        Self {
            id: transaction.id,
            donor_name: transaction.donor_name,
            value: transaction.value,
            date: transaction.date,
            currency: currency.into(),
        }
    }
}
