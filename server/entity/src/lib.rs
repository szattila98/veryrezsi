use sea_orm::prelude::Decimal;

pub mod account_activation;
pub mod currency;
pub mod expense;
pub mod predefined_expense;
pub mod recurrence;
pub mod transaction;
pub mod user;

pub type Id = u64;
pub type MoneyAmount = Decimal;
