use entity::{expense, transaction, Id, MoneyAmount};
use serde::{Deserialize, Serialize};
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
    pub currency_type_id: Id,
    pub recurrence_type_id: Id,
}

pub type ExpenseResponse = Vec<ExpenseWithTransactions>;

#[derive(Clone, Serialize, PartialEq, Eq, Debug)]
pub struct ExpenseWithTransactions {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub value: MoneyAmount,
    pub start_date: String,
    pub user_id: Id,
    pub currency_type_id: Id,
    pub recurrence_type_id: Id,
    pub predefined_expense_id: Option<Id>,
    pub transactions: Vec<transaction::Model>,
}

impl ExpenseWithTransactions {
    #[must_use]
    pub fn new(expense: expense::Model, transactions: Vec<transaction::Model>) -> Self {
        ExpenseWithTransactions {
            id: expense.id,
            name: expense.name,
            description: expense.description,
            value: expense.value,
            start_date: expense.start_date.to_string(),
            user_id: expense.user_id,
            currency_type_id: expense.currency_type_id,
            recurrence_type_id: expense.recurrence_type_id,
            predefined_expense_id: expense.predefined_expense_id,
            transactions,
        }
    }
}
