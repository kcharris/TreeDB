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
pub async fn find_items_by_parent_id(db_name: String, id: Option<i32>) -> Result<String, ItemDBError>{
    let db = get_db_conn(&db_name).await?;
    let res: String;
    let mut items: serde_json::Value;
    if id.is_none() {
      items = sea_orm::JsonValue::Array(item::Entity::find().filter(Expr::col(item::Column::ParentId).is_null()).into_json().all(&db).await?);
    }
    else{
      items = sea_orm::JsonValue::Array(item::Entity::find().filter(item::Column::ParentId.eq(id.unwrap())).into_json().all(&db).await?);
    }
    
    let res = items;
    Ok(res.to_string())
}

#[tauri::command]
pub async fn get_item_by_id(db_name: String, id: i32) -> Result<String, ItemDBError>{
    let db = get_db_conn(&db_name).await?;
    let mut items: serde_json::Value;
    
    let res = item::Entity::find_by_id(id).into_json().one(&db).await?.expect("Item not found with id").to_string();
    
    Ok(res)
}

#[tauri::command]
pub async fn add_item(db_name: String, payload: String) -> Result<Option<i32>, ItemDBError>{
    let db = get_db_conn(&db_name).await?;
    let json_item = serde_json::from_str(&payload).unwrap();
    let mut item = item::ActiveModel{..Default::default()};
    item.set_from_json(json_item)?;

    let res = item::Entity::insert(item).exec(&db).await?;
    Ok(Some(res.last_insert_id))
}

#[tauri::command]
pub async fn delete_item(db_name: String, id: i32) -> Result<(), ItemDBError>{
    let db = get_db_conn(&db_name).await?;
    let item1 = item::Entity::find_by_id(id).one(&db).await?.unwrap();
    let res = item1.delete(&db).await?;
    Ok(())
}

#[tauri::command]
pub async fn update_item(db_name: String, payload: String) -> Result<(), ItemDBError>{
    let db = get_db_conn(&db_name).await?;
    let json_item: serde_json::Value = serde_json::from_str(&payload).unwrap();
    let mut item = item::ActiveModel{..Default::default()};
    item.set_from_json(json_item.clone())?;
    let n: i32 = json_item["id"].as_i64().unwrap().try_into()?;
    item.id = ActiveValue::Set(n);
    
    let _res = item.save(&db).await?;
    Ok(())
}