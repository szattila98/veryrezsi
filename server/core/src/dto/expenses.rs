use entity::{Id, MoneyAmount};
use serde::Deserialize;
use validator::Validate;

#[derive(Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NewExpenseRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "expense name must not be empty or longer than 255 characters"
    ))]
    pub name: String,

    #[validate(length(
        max = 2000,
        message = "expense description must not be longer than 2000 characters"
    ))]
    pub description: String,

    pub recurrence_type_id: Id,

    pub currency_type_id: Id,

    pub predefined_expense_id: Option<Id>,

    pub start_date: String,

    pub value: MoneyAmount,
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NewPredefinedExpenseRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "predefined expense name must not be empty or longer than 255 characters"
    ))]
    pub name: String,
    #[validate(length(
        max = 2000,
        message = "predefined expense description must not be longer than 2000 characters"
    ))]
    pub description: String,
    pub value: MoneyAmount,
    pub currency_type_id: Id,
    pub recurrence_type_id: Id,
}
