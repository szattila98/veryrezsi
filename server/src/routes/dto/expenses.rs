#![allow(missing_docs)]

use fancy_regex::Regex;
use lazy_static::lazy_static;
use serde::Deserialize;
use validator::{Validate, ValidationError};
use entity::{Id, MoneyAmount};

lazy_static! {
    /// Date validation regex.
    static ref DATE_REGEX: Regex =
        Regex::new(r"/^\d{4}-(0[1-9]|1[0-2])-(0[1-9]|[12][0-9]|3[01])$/")
            .unwrap();
}

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NewExpenseRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "expense name must not be empty"
    ))]
    pub name: String,

    pub description: String,

	pub recurrence_type_id: Id,

	pub currency_type_id: Id,

	pub predefined_expense_id: Id,

	// TODO: Date format validator
	pub start_date: String,

	pub value: MoneyAmount,
}

fn validate_date(value: &str) -> Result<(), ValidationError> {
    if DATE_REGEX.is_match(value).unwrap() {
        return Ok(());
    }
    Err(ValidationError::new("start date must be in YYYY-MM-DD naive date format"))
}