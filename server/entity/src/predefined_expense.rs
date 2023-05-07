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
#[sea_orm(table_name = "predefined_expenses")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Id,
    pub name: String,
    pub description: String,
    pub value: MoneyAmount,
    pub currency_id: Id,
    pub recurrence_id: Id,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::currencies::Entity",
        from = "Column::CurrencyId",
        to = "super::currencies::Column::Id"
    )]
    Currency,
    #[sea_orm(
        belongs_to = "super::recurrences::Entity",
        from = "Column::RecurrenceId",
        to = "super::recurrences::Column::Id"
    )]
    Recurrence,
}
