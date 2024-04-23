use sea_orm::entity::prelude::*;
use crate::entities::item::serde::Serialize;
use crate::entities::item::serde::Deserialize;
use serde;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)] // Skip deserializing
    pub id: i32,
    pub name: String,
    pub parent: Option<i32>,
    pub priority: Option<i32>,
    pub availability: Option<TimeDate>,
    pub completed: Option<bool>,
    pub resource: Option<String>,
    pub est_time: Option<String>,
    pub start_time: Option<TimeDate>,
    pub end_time: Option<TimeDate>,
    pub description: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}


impl ActiveModelBehavior for ActiveModel {
    
}