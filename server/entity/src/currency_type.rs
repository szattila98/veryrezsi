use crate::Id;
use sea_orm::entity::prelude::*;
use serde::{self, Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "currency_type")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Id,
    #[sea_orm(unique)]
    pub abbreviation: String,
    #[sea_orm(unique)]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
