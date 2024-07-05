// code used from https://blog.moonguard.dev/how-to-use-local-sqlite-database-with-tauri
use std::fs;
use std::path::Path;

use crate::db::db_util::*;
use crate::entities::*;

use crate::migrator;
use sea_orm::{Database, DbErr, ActiveValue, ActiveModelTrait, EntityTrait};
use sea_orm_migration::prelude::*;

// Check if a database file exists, and create one if it does not.
pub fn init() {
    if !db_file_exists() {
        create_db_file();
    }
}

// create tables if they do not exist
pub async fn run_migrator() -> Result<(), Error>{
    let db = Database::connect("sqlite://".to_string() + &get_db_path().clone()).await?;
    let schema_manager = SchemaManager::new(&db);

    // starts a fresh migration if one of the tables does not exist
    if !schema_manager.has_table("item").await? || !schema_manager.has_table("item_tag").await? || !schema_manager.has_table("tag").await? {
        migrator::Migrator::fresh(&db).await?;
    }
    assert!(schema_manager.has_table("item").await?);
    assert!(schema_manager.has_table("tag").await?);
    assert!(schema_manager.has_table("item_tag").await?);

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
pub fn db_file_exists() -> bool {
    let db_path = get_db_path();
    Path::new(&db_path).exists()
}

// Get the path where the database file should be located.
pub fn get_db_path() -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/PersonalDB/database.sqlite"
}

