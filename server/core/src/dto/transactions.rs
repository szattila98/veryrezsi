use entity::{transaction, Id, MoneyAmount};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewTransactionRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "donor name must not be empty or longer than 255 characters"
    ))]
    pub donor_name: String,

    pub currency_type_id: Id,

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
    pub currency_type_id: Id,
    pub expense_id: Id,
}

impl From<transaction::Model> for TransactionResponse {
    fn from(transaction: transaction::Model) -> Self {
        Self {
            id: transaction.id,
            donor_name: transaction.donor_name,
            value: transaction.value,
            date: transaction.date,
            currency_type_id: transaction.currency_type_id,
            expense_id: transaction.expense_id,
        }
    }
}
