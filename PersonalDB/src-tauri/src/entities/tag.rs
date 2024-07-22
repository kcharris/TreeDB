use sea_orm::entity::prelude::*;
use crate::entities::tag::serde::Serialize;
use crate::entities::tag::serde::Deserialize;
use serde;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tag")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}


impl ActiveModelBehavior for ActiveModel {
    
}

impl Related<super::item::Entity> for Entity {
    
    fn to() -> RelationDef {
        super::item_tag::Relation::Item.def()
    }

    fn via() -> Option<RelationDef> {
        
        Some(super::item_tag::Relation::Tag.def().rev())
    }
}