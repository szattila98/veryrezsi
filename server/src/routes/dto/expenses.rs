#![allow(missing_docs)]

use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Deserialize, Validate)]
pub struct newExpenseRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "expense name must not be empty"
    ))]
    pub name: String,

    pub description: String,

	pub recurrence_type_id: i64,

	pub currency_type_id: i64,

	pub predefined_expense_id: i64,

	// TODO: Date format validator
	pub start_date: String,

	pub value: MoneyAmount,

}