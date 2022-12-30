use sea_orm::prelude::Decimal;

pub mod account_activation;
pub mod currency_type;
pub mod expense;
pub mod predefined_expense;
pub mod recurrence_type;
pub mod transaction;
pub mod user;

pub type Id = u64;
pub type MoneyAmount = Decimal;
