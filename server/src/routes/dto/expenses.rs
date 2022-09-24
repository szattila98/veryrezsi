#![allow(missing_docs)]

use entity::{Id, MoneyAmount};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NewExpenseRequest {
    #[validate(length(min = 1, max = 255, message = "expense name must not be empty"))]
    pub name: String,

    pub description: String,

    pub recurrence_type_id: Id,

    pub currency_type_id: Id,

    pub predefined_expense_id: Option<Id>,

    pub start_date: String,

    pub value: MoneyAmount,
}
