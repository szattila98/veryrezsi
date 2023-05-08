use entity::{currency, expense, predefined_expense, recurrence, Id, MoneyAmount};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    currencies::CurrencyResponse,
    recurrences::RecurrenceResponse,
    transactions::{TransactionResponse, TransactionResponseParts},
};

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

#[derive(Clone, Serialize, PartialEq)]
pub struct PredefinedExpenseResponse {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub value: MoneyAmount,
    pub currency: CurrencyResponse,
    pub recurrence: RecurrenceResponse,
}

pub type PredefinedExpenseResponseParts = (
    predefined_expense::Model,
    currency::Model,
    recurrence::Model,
);
impl<'a> From<PredefinedExpenseResponseParts> for PredefinedExpenseResponse {
    fn from((predefined_expense, currency, recurrence): PredefinedExpenseResponseParts) -> Self {
        Self {
            id: predefined_expense.id,
            name: predefined_expense.name,
            description: predefined_expense.description,
            value: predefined_expense.value,
            currency: currency.into(),
            recurrence: recurrence.into(),
        }
    }
}

#[derive(Clone, Serialize, PartialEq)]
pub struct ExpenseResponse {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub value: MoneyAmount,
    pub start_date: String,
    pub user_id: Id,
    pub currency: CurrencyResponse,
    pub recurrence: RecurrenceResponse,
    pub predefined_expense: Option<PredefinedExpenseResponse>,
    pub transactions: Vec<TransactionResponse>,
}

pub type ExpenseResponseParts = (
    expense::Model,
    currency::Model,
    recurrence::Model,
    Option<PredefinedExpenseResponseParts>,
    Vec<TransactionResponseParts>,
);
impl From<ExpenseResponseParts> for ExpenseResponse {
    fn from(
        (expense, currency, recurrence, predefined_expense, transactions): ExpenseResponseParts,
    ) -> Self {
        Self {
            id: expense.id,
            name: expense.name,
            description: expense.description,
            value: expense.value,
            start_date: expense.start_date.to_string(),
            user_id: expense.user_id,
            currency: currency.into(),
            recurrence: recurrence.into(),
            predefined_expense: predefined_expense
                .map(|predefined_expense| predefined_expense.into()),
            transactions: transactions
                .into_iter()
                .map(|transaction| transaction.into())
                .collect(),
        }
    }
}
