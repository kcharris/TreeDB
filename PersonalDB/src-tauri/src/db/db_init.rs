// code used from https://blog.moonguard.dev/how-to-use-local-sqlite-database-with-tauri
use std::fs;
use std::path::Path;
use sea_orm::{Database, DbErr};

use crate::migrator;
use crate::db::db_util::*;
use crate::entities::*;
use sea_orm_migration::prelude::*;
use sea_orm::ActiveValue;
use sea_orm::ActiveModelTrait;
use sea_orm::EntityTrait;
use crate::db::db_util::*;

// Check if a database file exists, and create one if it does not.
pub fn init() {
    if !db_file_exists() {
        create_db_file();
    }
}

// test connection to Sqlite
pub async fn run_migrator() -> Result<(), DbErr>{
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
pub fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/PersonalDB/database.sqlite"
}

