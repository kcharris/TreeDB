
use std::env::Args;
use std::fs;
use std::path::Path;
use thiserror::Error;
use sea_orm::{ColumnTrait, DatabaseConnection};
use sea_orm::{Database, DbErr};

use crate::migrator;
use crate::entities::*;
use sea_orm_migration::prelude::*;
use sea_orm::ActiveValue;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use crate::db::db_init::*;

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

pub async fn test_insert() -> Result<(), DbErr>{
    let db = get_db_conn().await?;
    let item1 = item::ActiveModel {
        name: ActiveValue::Set("first item".to_owned()),
        priority: ActiveValue::Set(Some(88)),
        ..Default::default()
    };
    // let item1 = item1.insert(&db).await?;
    let res = item::Entity::insert(item1).exec(&db).await?;
    let items: Vec<item::Model> = item::Entity::find().all(&db).await?;
    assert_eq!(items.len(), 1);
    Ok(())
}

#[tauri::command]
pub async fn find_items_by_parent_id(id: Option<i32>) -> Result<Vec<serde_json::Value>, Error>{
    let db = get_db_conn().await?;
    let items: Vec<serde_json::Value> = item::Entity::find().filter(item::Column::Parent.eq(id)).into_json().all(&db).await?;
    Ok(items)
}