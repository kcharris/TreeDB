// code used from https://blog.moonguard.dev/how-to-use-local-sqlite-database-with-tauri
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use crate::errors::ItemDBError;
use crate::migrator;
use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::prelude::*;
use serde_json::json;


// Check if a database file exists, and create one if it does not.
pub async fn init() -> Result<(), ItemDBError> {
    db_name_init();
    let db_name = get_db_name();
    assert_eq!(db_name, "default");
    if !db_file_exists(db_name.clone()){
        update_db_name_file(db_name.clone());
        create_db_file(db_name.clone());
        run_migrator(db_name.clone()).await?;
    }
    db_name_init();

    Ok(())
}

// check whether the db_name json file exists, and creates it if not
pub fn db_name_init(){
    let home_dir = dirs::home_dir().unwrap();
    let binding = home_dir.to_str().unwrap().to_owned() + "/.config/PersonalDB/db_name.json";
    let path = Path::new(&binding);
    if !path.exists(){
        fs::File::create(home_dir.to_str().unwrap().to_string() + "/.config/PersonalDB/db_name.json").unwrap();
    }
    update_db_name_file("default".to_string());
}

// Updates the filename stored in db_name.json to the given string
pub fn update_db_name_file(db_name: String){
    let home_dir = dirs::home_dir().unwrap();
    let json = json!({
        "name": db_name
    });
    fs::write(home_dir.to_str().unwrap().to_string() + "/.config/PersonalDB/db_name.json", json.to_string()).expect("Failed to write to file in update_db_name_file");
}

// Get's the current db's name being used for the item list from the db_name.json file
pub fn get_db_name() -> String {
    let home_dir = dirs::home_dir().unwrap();
    let mut file = fs::File::open(home_dir.to_str().unwrap().to_string() + "/.config/PersonalDB/db_name.json").expect("Unable to open file from get_db_name");
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let file_json:serde_json::Value = serde_json::from_str(&contents).unwrap();

    return file_json["name"].as_str().unwrap().to_string();
}

// Get a database connection using the apps default path
pub async fn get_db_conn(db_name: &str) -> Result<DatabaseConnection, DbErr> {
    let db:DatabaseConnection = Database::connect("sqlite://".to_string() + &get_db_path(db_name).clone()).await?;
    return Ok(db);
}

// create tables if they do not exist
pub async fn run_migrator(db_name: String) -> Result<(), ItemDBError>{
    let db = get_db_conn(&db_name).await?;
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
pub fn create_db_file(db_name: String) {
    let db_path = get_db_path(&db_name);
    let db_dir = Path::new(&db_path).parent().unwrap();

    // If the parent directory does not exist, create it.
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    // Create the database file if it does not already exist.
    if !(Path::new(&db_path)).exists(){
        fs::File::create(db_path).unwrap();
    }
}

// Check whether the database file exists.
pub fn db_file_exists(db_name: String) -> bool {
    let db_path = get_db_path(&db_name);
    Path::new(&db_path).exists()
}

// Get the path where the database file should be located.
pub fn get_db_path(db_name: &str) -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/PersonalDB/" + &db_name + ".sqlite"
}

// Deletes the sql file with the given name if it exists
pub fn delete_db_file(db_name: String) {
    if db_file_exists(db_name.clone()) {
        let db_path = get_db_path(&db_name);
        let db_path = Path::new(&db_path);
        let _ = fs::remove_file(db_path).unwrap();
    }
}

// Clones the db with the given name, the new db will have the clone name
pub async fn clone_db_file(db_name: String, clone_name: String) -> Result<(), ItemDBError>{
    if !db_file_exists(clone_name.clone()){
        create_db_file(clone_name.clone());
        let db_path = get_db_path(&db_name);
        let clone_path = get_db_path(&clone_name);
        fs::copy(db_path, clone_path)?;
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::item;
    use crate::entities::tag;
    use crate::entities::item_tag;
    use sea_orm::ActiveValue;
    use sea_orm::{EntityTrait, ActiveModelTrait};



    // test with "$ cargo test -- --test-threads=1 ", this is because of issues with async functions sharing resources
    // Sets up a test database to avoid over-writing original and get the connection
    pub async fn setup() -> Result<(), ItemDBError> {
        delete_db_file("test_database".to_string());
        create_db_file("test_database".to_string());
        run_migrator("test_database".to_string()).await?;

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

