
use std::env::Args;
use std::fs;
use std::path::Path;
use thiserror::Error;
use sea_orm::{ColumnTrait, DatabaseConnection};
use sea_orm::{Database, DbErr, InsertResult};

use crate::migrator;
use crate::entities::*;
use sea_orm_migration::prelude::*;
// use sea_orm_migration::prelude::ColumnSpec::Default;
use sea_orm::{ActiveValue, ActiveModelTrait, EntityTrait, QueryFilter} ;
use crate::db::db_init::*;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error("Error accessing or modifying database")]
  DbErr(#[from] sea_orm::DbErr)
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    serializer.serialize_str(self.to_string().as_ref())
  }
}

async fn get_db_conn() -> Result<DatabaseConnection, DbErr> {
    let db:DatabaseConnection = Database::connect("sqlite://".to_string() + &get_db_path().clone()).await?;
    return Ok(db);
}

pub async fn test_insert() -> Result<(), Error>{
    let db = get_db_conn().await?;
    add_item(r#"{"name": "cat"}"#.to_owned()).await?;
    add_item(r#"{"name": "dog"}"#.to_owned()).await?;
    add_item(r#"{"name": "butt"}"#.to_owned()).await?;
    add_item(r#"{"name": ""}"#.to_owned()).await?;
    let item1 = item::ActiveModel {
        name: ActiveValue::Set("first item".to_owned()),
        priority: ActiveValue::Set(Some(88)),
        ..Default::default()
    };
    // let item1 = item1.insert(&db).await?;
    let res = item::Entity::insert(item1).exec(&db).await?;
    let items: Vec<item::Model> = item::Entity::find().all(&db).await?;
    assert_eq!(items.len(), 5);
    Ok(())
}

#[tauri::command]
pub async fn find_items_by_parent_id(id: Option<i32>) -> Result<String, Error>{
    let db = get_db_conn().await?;
    let res: String;
    let mut items: serde_json::Value;
    if id.is_none() {
      items = sea_orm::JsonValue::Array(item::Entity::find().filter(Expr::col(item::Column::Parent).is_null()).into_json().all(&db).await?);
    }
    else{
      items = sea_orm::JsonValue::Array(item::Entity::find().filter(item::Column::Parent.eq(id.unwrap())).into_json().all(&db).await?);
    }
    
    let res = items;
    Ok(res.to_string())
}

#[tauri::command]
pub async fn add_item(payload: String) -> Result<Option<i32>, Error>{
  // the payload has to include the parent objects integer or nothing. I guess it's not a problem now but I need to ability to add the 
    let db = get_db_conn().await?;
    let json_item = serde_json::from_str(&payload).unwrap();
    let mut item = item::ActiveModel{..Default::default()};
    item.set_from_json(json_item)?;

    let res = item::Entity::insert(item).exec(&db).await?;
    Ok(Some(res.last_insert_id))
}