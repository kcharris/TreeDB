use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key)]
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