use std::env::Args;
use std::fs;
use std::path::Path;
use crate::errors::ItemDBError;

use crate::migrator;
use crate::entities::*;
use sea_orm_migration::prelude::*;
use sea_orm::{ActiveValue, ActiveModelTrait, EntityTrait, QueryFilter, ModelTrait, ColumnTrait, DatabaseConnection, Database, InsertResult};
use crate::db::db_util::*;
use serde_json::json;

#[tauri::command]
pub async fn add_tag(name: String) -> Result<Option<i32>, ItemDBError> {
  let db = get_db_conn().await?;
  let mut tag = tag::ActiveModel{..Default::default()};
  tag.name = ActiveValue::set(name);

  let res = tag::Entity::insert(tag).exec(&db).await?;
  Ok(Some(res.last_insert_id))
}

#[tauri::command]
pub async fn find_tags() -> Result<String, ItemDBError>{
    let db = get_db_conn().await?;
    let res: String;
    let mut tags: serde_json::Value;

    tags = sea_orm::JsonValue::Array(tag::Entity::find().into_json().all(&db).await?);
    
    let res = tags;
    Ok(res.to_string())
}

#[tauri::command]
pub async fn update_tag(payload: String) -> Result<(), ItemDBError>{
  let db = get_db_conn().await?;
  let json_item: serde_json::Value = serde_json::from_str(&payload).unwrap();
  let mut item = item::ActiveModel{..Default::default()};
  item.set_from_json(json_item.clone())?;
  let n: i32 = json_item["id"].as_i64().unwrap().try_into()?;
  item.id = ActiveValue::Set(n);
  
  let _res = item.save(&db).await?;
  Ok(())
}

#[tauri::command]
pub async fn delete_tag(id: i32) -> Result<(), ItemDBError>{
    let db = get_db_conn().await?;
    let tag = item::Entity::find_by_id(id).one(&db).await?.unwrap();
    let res = tag.delete(&db).await?;
    Ok(())
}