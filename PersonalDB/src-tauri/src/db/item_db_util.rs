use std::env::Args;
use std::fs;
use std::path::Path;
use crate::errors::ItemDBError;

use crate::migrator;
use crate::entities::*;
use sea_orm_migration::prelude::*;
use sea_orm::{ActiveValue, ActiveModelTrait, EntityTrait, QueryFilter, ModelTrait, ColumnTrait, DatabaseConnection, Database, InsertResult, DeleteResult};
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
pub async fn delete_item(db_name: String, id: i32) -> Result<u64, ItemDBError>{
    let db = get_db_conn(&db_name).await?;
    let item1 = item::Entity::find_by_id(id).one(&db).await?.unwrap();
    let res = item1.delete(&db).await?;
    Ok(res.rows_affected)
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

#[cfg(test)]
mod tests {
    use super::*;

    // test with "$ cargo test -- --test-threads=1 ", this is because of issues with async functions sharing resources
    // Sets up a test database to avoid over-writing original
    pub async fn setup() -> Result<(), ItemDBError> {
        delete_db_file("test_database");
        create_db_file("test_database");
        run_migrator("test_database").await?;

        return Ok(());
    }

    #[async_std::test]
    pub async fn test_async_item() -> Result<(), ItemDBError> {
        setup().await?;
        let db_name = "test_database".to_owned();

        // test insert
        let json_str = r#"{
            "name": "tester1"
        }"#.to_owned();

        let res = add_item(db_name.clone(), json_str).await?;
        assert_eq!(res, Some(1), "item insert failed");
    

        // test get
        let payload = get_item_by_id(db_name.clone(), 1).await?;
        let json_item = serde_json::from_str(&payload).unwrap();

        let mut item = item::ActiveModel{..Default::default()};
        item.set_from_json(json_item)?;

        assert_eq!(item.name, sea_orm::ActiveValue::Set("tester1".to_owned()), "item get failed");


        // test update
        let json_str = r#"{
            "name": "updated1",
            "id": 1
        }"#.to_owned();

        update_item(db_name.clone(), json_str).await?;

        let payload = get_item_by_id(db_name.clone(), 1).await?;
        let json_item = serde_json::from_str(&payload).unwrap();

        let mut item = item::ActiveModel{..Default::default()};
        item.set_from_json(json_item)?;

        assert_eq!(item.name, sea_orm::ActiveValue::Set("updated1".to_owned()), "item update failed");
        

        // test delete
        let db_name = "test_database".to_owned();
        let res = delete_item(db_name, 1).await?;

        assert_eq!(res, 1, "delete failed");
        Ok(())
    }
}