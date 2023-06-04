use crate::Id;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    DeriveActiveModelBehavior,
    DeriveEntityModel,
    Deserialize,
    Serialize,
)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Id,
    #[sea_orm(unique)]
    pub email: String,
    pub username: String,
    pub pw_hash: String,
    #[sea_orm(default_value = "false")]
    pub activated: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
