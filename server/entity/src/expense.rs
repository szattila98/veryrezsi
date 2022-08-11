use crate::{Id, Value};
use sea_orm::entity::prelude::*;
use serde::{self, Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "predefined_expense")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Id,
    pub name: String,
    pub description: String,
    pub value: Value,
    pub start_date: DateTimeLocal,
    pub user_id: Id,
    pub currency_type_id: Id,
    pub recurrence_type_id: Id,
    #[sea_orm(nullable)]
    pub predefined_expense_id: Option<Id>,
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

impl ActiveModelBehavior for ActiveModel {}
