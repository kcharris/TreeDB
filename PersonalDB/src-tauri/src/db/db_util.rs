// code used from https://blog.moonguard.dev/how-to-use-local-sqlite-database-with-tauri
use std::fs;
use std::path::Path;
use std::time::Duration;
use serde_json::json;
use crate::errors::ItemDBError;
use futures::executor::block_on;

use async_std::prelude::*;
use async_std::task;
use crate::entities::*;

use crate::migrator;
use sea_orm::{Database, DatabaseConnection, DbErr, ActiveValue, ActiveModelTrait, EntityTrait};
use sea_orm_migration::prelude::*;
use sea_orm_migration::MigrationStatus;


// Check if a database file exists, and create one if it does not.
pub async fn init() -> Result<(), ItemDBError> {
    let home_dir = dirs::home_dir().unwrap();
    let paths = fs::read_dir(home_dir.to_str().unwrap().to_string() + "/.config/PersonalDB/").unwrap();
    let num_files = paths.count();
    if num_files < 1{
        create_db_file("default");
        run_migrator("default").await;
    }
    Ok(())
}

// Get a database connection using the apps default path
pub async fn get_db_conn(db_name: &str) -> Result<DatabaseConnection, DbErr> {
    let db:DatabaseConnection = Database::connect("sqlite://".to_string() + &get_db_path(db_name).clone()).await?;
    return Ok(db);
}

// create tables if they do not exist
pub async fn run_migrator(db_name: &str) -> Result<(), ItemDBError>{
    let db = get_db_conn(db_name).await?;
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
pub fn create_db_file(db_name: &str) {
    let db_path = get_db_path(db_name);
    let db_dir = Path::new(&db_path).parent().unwrap();

    // If the parent directory does not exist, create it.
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    // Create the database file.
    fs::File::create(db_path).unwrap();
}

// Check whether the database file exists.
pub fn db_file_exists(db_name: &str) -> bool {
    let db_path = get_db_path(db_name);
    Path::new(&db_path).exists()
}

// Get the path where the database file should be located.
pub fn get_db_path(db_name: &str) -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/PersonalDB/" + &db_name + ".sqlite"
}

pub fn delete_db_file(db_name: &str) {
    if db_file_exists(db_name) {
        let db_path = get_db_path(db_name);
        let db_path = Path::new(&db_path);
        let _ = fs::remove_file(db_path).unwrap();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // test with "$ cargo test -- --test-threads=1 ", this is because of issues with async functions sharing resources
    // Sets up a test database to avoid over-writing original and get the connection
    pub async fn setup() -> Result<(), ItemDBError> {
        delete_db_file("test_database");
        create_db_file("test_database");
        run_migrator("test_database").await?;

        return Ok(());
    }

    #[async_std::test]
    async fn test_insert_basic() -> Result<(), ItemDBError> {
        setup().await?;
        // test basic insert
        let db = get_db_conn("test_database").await?;
        for i in 0..=5 {
            let mut item = item::ActiveModel{..Default::default()};
            item.name = ActiveValue::Set(format!("itemy-{i}"));
            let res = item::Entity::insert(item).exec(&db).await?;
        }
        let mut item = item::ActiveModel{..Default::default()};
        item.name = ActiveValue::Set(format!("itemy-55"));
        let res = item::Entity::insert(item).exec(&db).await?;

        assert_eq!(res.last_insert_id, 7);

        for i in 0..=5 {
            let mut tag = tag::ActiveModel{..Default::default()};
            tag.name = ActiveValue::Set(format!("taggy-{i}"));
            let res = tag::Entity::insert(tag).exec(&db).await?;
        }
        let mut tag = tag::ActiveModel{..Default::default()};
        tag.name = ActiveValue::Set(format!("taggy-77"));
        let res = tag::Entity::insert(tag).exec(&db).await?;
        
        assert_eq!(res.last_insert_id, 7);
  
        
        // test junction
        for i in 1..=5{
            for j in 1..=5{
                let mut item_tag = item_tag::ActiveModel::from_json(json!({
                    "item_id": i,
                    "tag_id": j
                }))?;
                let res = item_tag::Entity::insert(item_tag).exec(&db).await?;
            }
        }
        let mut item_tag = item_tag::ActiveModel::from_json(json!({
            "item_id": 6,
            "tag_id": 6
        }))?;
        let res = item_tag::Entity::insert(item_tag).exec(&db).await?;
        assert_eq!(res.last_insert_id, (6,6));
        Ok(())
    }
}

