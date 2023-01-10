use crate::{transaction, Id, MoneyAmount};
use entity::expense;
use serde::{self, Deserialize, Serialize};

pub type ExpenseResponse = Vec<ExpenseWithTransactions>;

struct ExpenseWithTransactions {
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

impl ExpenseWithTransactions {
    fn from(&self, expense: expense::Model, transactions: Vec<transaction::Model>) -> self {
        &self.id = *expense.id;
        &self.name = *expense.name;
        &self.description = *expense.description;
        &self.value = *expense.value;
        &self.start_date = *expense.start_date;
        &self.user_id = *expense.user_id;
        &self.currency_type_id = *expense.currency_type_id;
        &self.recurrence_type_id = *expense.recurrence_type_id;
        &self.predefined_expense_id = *expense.predefined_expense_id;
        &self.transactions = *transactions;
    }
}
