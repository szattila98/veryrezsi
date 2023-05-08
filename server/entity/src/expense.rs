use crate::{Id, MoneyAmount};
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
    pub currency_id: Id,
    pub recurrence_id: Id,
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
    #[sea_orm(has_many = "super::transaction::Entity")]
    Transaction,
    #[sea_orm(
        belongs_to = "super::currency::Entity",
        from = "Column::CurrencyId",
        to = "super::currency::Column::Id"
    )]
    Currency,
    #[sea_orm(
        belongs_to = "super::recurrence::Entity",
        from = "Column::RecurrenceId",
        to = "super::recurrence::Column::Id"
    )]
    Recurrence,
    #[sea_orm(
        belongs_to = "super::predefined_expense::Entity",
        from = "Column::PredefinedExpenseId",
        to = "super::predefined_expense::Column::Id"
    )]
    PredefinedExpense,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::transaction::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Transaction.def()
    }
}

impl Related<super::currency::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Currency.def()
    }
}

impl Related<super::recurrence::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Recurrence.def()
    }
}

impl Related<super::predefined_expense::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PredefinedExpense.def()
    }
}
