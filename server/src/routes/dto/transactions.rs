#![allow(missing_docs)]

use entity::{Id, MoneyAmount};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
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
