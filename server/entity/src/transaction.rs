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
#[sea_orm(table_name = "transactions")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Id,
    pub donor_name: String,
    pub value: MoneyAmount,
    pub date: DateTimeLocal,
    pub currency_type_id: Id,
    pub expense_id: Id,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::currency_type::Entity",
        from = "Column::CurrencyTypeId",
        to = "super::currency_type::Column::Id"
    )]
    CurrencyType,
    #[sea_orm(
        belongs_to = "super::expense::Entity",
        from = "Column::ExpenseId",
        to = "super::expense::Column::Id"
    )]
    Expense,
}
