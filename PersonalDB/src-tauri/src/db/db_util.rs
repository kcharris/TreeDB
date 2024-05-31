
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
use sea_orm::{ActiveValue, ActiveModelTrait, EntityTrait, QueryFilter, ModelTrait};
use crate::db::db_init::*;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error("Error accessing or modifying database")]
  DbErr(#[from] sea_orm::DbErr),
  #[error("Error trying from integer")]
  TryFromIntError(#[from] std::num::TryFromIntError)
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
    for i in 1..=5{
      let item1 = item::ActiveModel {
        name: ActiveValue::Set(format!("item{i}").to_owned()),
        priority: ActiveValue::Set(Some(88 + i)),
        ..Default::default()
    };
    // let item1 = item1.insert(&db).await?;
    let res = item::Entity::insert(item1).exec(&db).await?;
    }
    
    let items: Vec<item::Model> = item::Entity::find().all(&db).await?;
    //assert_eq!(items.len(), 5);
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
    let db = get_db_conn().await?;
    let json_item = serde_json::from_str(&payload).unwrap();
    let mut item = item::ActiveModel{..Default::default()};
    item.set_from_json(json_item)?;

    let res = item::Entity::insert(item).exec(&db).await?;
    Ok(Some(res.last_insert_id))
}

#[tauri::command]
pub async fn delete_item(id: i32) -> Result<(), Error>{
    let db = get_db_conn().await?;
    let item1 = item::Entity::find_by_id(id).one(&db).await?.unwrap();
    let res = item1.delete(&db).await?;
    Ok(())
}

#[tauri::command]
pub async fn update_item(payload: String) -> Result<(), Error>{
    let db = get_db_conn().await?;
    let json_item: serde_json::Value = serde_json::from_str(&payload).unwrap();
    let mut item = item::ActiveModel{..Default::default()};
    item.set_from_json(json_item.clone())?;
    let n: i32 = json_item["id"].as_i64().unwrap().try_into()?;
    item.id = ActiveValue::Set(n);
    
    let _res = item.save(&db).await?;
    Ok(())
}