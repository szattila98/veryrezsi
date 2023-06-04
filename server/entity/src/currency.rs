use crate::Id;
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
#[sea_orm(table_name = "currencies")]
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
