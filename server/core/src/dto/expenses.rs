use entity::{expense, transaction, Id, MoneyAmount};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::transactions::TransactionResponse;

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

    pub currency_id: Id,

    pub recurrence_id: Id,

    pub predefined_expense_id: Option<Id>,

    pub start_date: String,

    pub value: MoneyAmount,
}

#[derive(Clone, Deserialize, Validate)]
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
    pub currency_id: Id,
    pub recurrence_id: Id,
}

#[derive(Clone, Serialize, PartialEq, Eq)]
pub struct ExpenseResponse {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub value: MoneyAmount,
    pub start_date: String,
    pub user_id: Id,
    pub currency_id: Id,
    pub recurrence_id: Id,
    pub predefined_expense_id: Option<Id>,
    pub transactions: Vec<TransactionResponse>,
}

impl From<(expense::Model, Vec<transaction::Model>)> for ExpenseResponse {
    fn from((expense, transactions): (expense::Model, Vec<transaction::Model>)) -> Self {
        Self {
            id: expense.id,
            name: expense.name,
            description: expense.description,
            value: expense.value,
            start_date: expense.start_date.to_string(),
            user_id: expense.user_id,
            currency_id: expense.currency_id,
            recurrence_id: expense.recurrence_id,
            predefined_expense_id: expense.predefined_expense_id,
            transactions: transactions
                .into_iter()
                .map(|transaction| transaction.into())
                .collect(),
        }
    }
}
