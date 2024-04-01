// code used from https://blog.moonguard.dev/how-to-use-local-sqlite-database-with-tauri
use std::fs;
use std::path::Path;
use sea_orm::{Database, DbErr};

use crate::migrator;
use crate::entities::*;
use sea_orm_migration::prelude::*;
use sea_orm::ActiveValue;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;

use crate::migrator::m20220101_000001_create_item_table::Item;

// Check if a database file exists, and create one if it does not.
pub fn init() {
    if !db_file_exists() {
        create_db_file();
    }
}

// establish connection to Sqlite
pub async fn establish_sql_connection() -> Result<(), DbErr>{
    let db = Database::connect("sqlite://".to_string() + &get_db_path().clone()).await?;
    let schema_manager = SchemaManager::new(&db);
    migrator::Migrator::refresh(&db).await?;
    assert!(schema_manager.has_table("item").await?);
    test_insert().await?;
    Ok(())
}

// Create the database file.
fn create_db_file() {
    let db_path = get_db_path();
    let db_dir = Path::new(&db_path).parent().unwrap();

    // If the parent directory does not exist, create it.
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    // Create the database file.
    fs::File::create(db_path).unwrap();
}

// Check whether the database file exists.
fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

// Get the path where the database file should be located.
fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/PersonalDB/database.sqlite"
}

pub async fn test_insert() -> Result<(), DbErr>{
    let db = Database::connect("sqlite://".to_string() + &get_db_path().clone()).await?;
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