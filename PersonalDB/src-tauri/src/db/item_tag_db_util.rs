use std::env::Args;
use std::fs;
use std::path::Path;
use crate::errors::ItemDBError;

use crate::migrator;
use crate::entities::*;
use sea_orm_migration::prelude::*;
use sea_orm::{ActiveValue, ActiveModelTrait, EntityTrait, QueryFilter, ModelTrait, ColumnTrait, DatabaseConnection, Database, InsertResult};
use crate::db::db_util::*;
use crate::db::item_db_util::*;
use crate::db::tag_db_util::*;
use serde_json::json;

//template for item tag

#[tauri::command]
pub async fn add_item_tag(item_id:i32, tag_id:i32) -> Result<(), ItemDBError> {
  let db = get_db_conn().await?;
  let mut item_tag = item_tag::ActiveModel{..Default::default()};
  item_tag.item_id = ActiveValue::set(item_id);
  item_tag.tag_id = ActiveValue::set(tag_id);

  let res = item_tag::Entity::insert(item_tag).exec(&db).await?;
  Ok(())
}

#[tauri::command]
pub async fn find_item_tags_by_item_id(id:i32) -> Result<String, ItemDBError>{
    let db = get_db_conn().await?;
    let res: String;
    let mut item_tags: serde_json::Value;

    item_tags = sea_orm::JsonValue::Array(item_tag::Entity::find().filter(item_tag::Column::ItemId.eq(id)).into_json().all(&db).await?);

    let res = item_tags;
    Ok(res.to_string())
}

#[tauri::command]
pub async fn find_item_tags_by_tag_id(id:i32) -> Result<String, ItemDBError>{
    let db = get_db_conn().await?;
    let res: String;
    let mut item_tags: serde_json::Value;

    item_tags = sea_orm::JsonValue::Array(item_tag::Entity::find().filter(item_tag::Column::TagId.eq(id)).into_json().all(&db).await?);

    let res = item_tags;
    Ok(res.to_string())
}

#[tauri::command]
pub async fn delete_item_tag(item_id: i32, tag_id:i32) -> Result<(), ItemDBError>{
    let db = get_db_conn().await?;
    let item_tag = item_tag::Entity::find_by_id((item_id, tag_id)).one(&db).await?.unwrap();
    let res = item_tag.delete(&db).await?;
    Ok(())
}