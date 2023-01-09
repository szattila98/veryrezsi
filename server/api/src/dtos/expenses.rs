use crate::{transaction, Id, MoneyAmount};
use serde::{self, Deserialize, Serialize};

pub struct Expense {
    pub id: Id,
    pub name: String,
    pub description: String,
    pub value: MoneyAmount,
    pub start_date: Date,
    pub user_id: Id,
    pub currency_type_id: Id,
    pub recurrence_type_id: Id,
    pub predefined_expense_id: Option<Id>,
    pub transactions: Vec<transaction::Model>,
}
