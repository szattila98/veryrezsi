use crate::{transaction, Id, MoneyAmount};
use sea_orm::entity::prelude::*;
use serde::{self, Deserialize, Serialize};

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    DeriveEntityModel,
    DeriveActiveModelBehavior,
    Deserialize,
    Serialize,
)]
#[sea_orm(table_name = "expenses")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Id,
    pub name: String,
    pub description: String,
    pub value: MoneyAmount,
    pub start_date: Date,
    pub user_id: Id,
    pub currency_type_id: Id,
    pub recurrence_type_id: Id,
    #[sea_orm(nullable)]
    pub predefined_expense_id: Option<Id>,
    pub transactions: Vec<transaction::Model>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::currency_type::Entity",
        from = "Column::CurrencyTypeId",
        to = "super::currency_type::Column::Id"
    )]
    CurrencyType,
    #[sea_orm(
        belongs_to = "super::recurrence_type::Entity",
        from = "Column::RecurrenceTypeId",
        to = "super::recurrence_type::Column::Id"
    )]
    RecurrenceType,
    #[sea_orm(
        belongs_to = "super::predefined_expense::Entity",
        from = "Column::PredefinedExpenseId",
        to = "super::predefined_expense::Column::Id"
    )]
    PredefinedExpense,
}
