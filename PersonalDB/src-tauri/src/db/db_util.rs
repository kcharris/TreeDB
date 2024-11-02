// code used from https://blog.moonguard.dev/how-to-use-local-sqlite-database-with-tauri
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use crate::errors::ItemDBError;
use crate::migrator;
use sea_orm::{Database, DatabaseConnection, DbErr};
use sea_orm_migration::prelude::*;
use crate::db::default_db_util::*;
use serde_json::json;
use regex::Regex;


/// Makes sure required app files exist, if not creates them.
pub async fn init() -> Result<(), ItemDBError> {
    let home_dir = dirs::home_dir().unwrap();
    
    // make sure there is a main folder with subfolders for the app
    let binding = home_dir.to_str().unwrap().to_owned() + "/.config/TreeDB/";
    let path = Path::new(&binding);
    if !path.exists(){
        let _ = fs::create_dir_all(path);
    }

    // make sure the app contains a json file to store the default database name
    let binding = home_dir.to_str().unwrap().to_owned() + "/.config/TreeDB/db_name.json";
    let path = Path::new(&binding);
    if !path.exists(){
        fs::File::create(home_dir.to_str().unwrap().to_string() + "/.config/TreeDB/db_name.json").unwrap();
        update_on_start_db("default".to_string());
    }

    // make sure the app contains a json file to store a first start value.
    let binding = home_dir.to_str().unwrap().to_owned() + "/.config/TreeDB/first_start.json";
    let path = Path::new(&binding);
    if !path.exists(){
        fs::File::create(home_dir.to_str().unwrap().to_string() + "/.config/TreeDB/first_start.json").unwrap();
        update_first_start(0);
    }

    // make sure there is a folder for backups
    let backup_binding = home_dir.to_str().unwrap().to_owned() + "/.config/TreeDB/backups";
    let path = Path::new(&backup_binding);
    if !path.exists(){
        let _ = fs::create_dir(path);
    }

    // On first start create an Example DB file named Example
    if get_first_start() == 0{
        let db_name = "Example".to_string();
        update_on_start_db(db_name.clone());
        create_db_file(db_name.clone()).await;
        run_migrator(db_name.clone()).await?;
        set_default_values(db_name.clone()).await?;
        update_first_start(1);
    }

    Ok(())
}

/// Updates the filename stored in db_name.json to the given string
#[tauri::command]
pub fn update_on_start_db(db_name: String){
    let home_dir = dirs::home_dir().unwrap();
    let json = json!({
        "name": db_name
    });
    fs::write(home_dir.to_str().unwrap().to_string() + "/.config/TreeDB/db_name.json", json.to_string()).expect("Failed to write to file in update_db_name_file");
}

/// Get's the current db's name being used for the item list from the db_name.json file
#[tauri::command]
pub fn get_db_name() -> String {
    let home_dir = dirs::home_dir().unwrap();
    let mut file = fs::File::open(home_dir.to_str().unwrap().to_string() + "/.config/TreeDB/db_name.json").expect("Unable to open file from get_db_name");
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let file_json:serde_json::Value = serde_json::from_str(&contents).unwrap();

    return file_json["name"].as_str().unwrap().to_string();
}

pub fn get_first_start()-> i64{
    let home_dir = dirs::home_dir().unwrap();
    let mut file = fs::File::open(home_dir.to_str().unwrap().to_string() + "/.config/TreeDB/first_start.json").expect("Unable to open file from get_first_start");
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let file_json:serde_json::Value = serde_json::from_str(&contents).unwrap();

    return file_json["val"].as_i64().unwrap();
}

pub fn update_first_start(n: i32){
    let home_dir = dirs::home_dir().unwrap();
    let json = json!({
        "val": n
    });
    fs::write(home_dir.to_str().unwrap().to_string() + "/.config/TreeDB/first_start.json", json.to_string()).expect("Failed to write to file in update_first_start");
}

/// Get a database connection using the apps default path
pub async fn get_db_conn(db_name: &str) -> Result<DatabaseConnection, DbErr> {
    let db:DatabaseConnection = Database::connect("sqlite://".to_string() + &get_db_path(db_name).clone()).await?;
    return Ok(db);
}

/// create tables if they do not exist
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

/// Create the database file.
#[tauri::command]
pub async fn create_db_file(db_name: String) {
    let db_path = get_db_path(&db_name);
    let db_dir = Path::new(&db_path).parent().unwrap();

    // If the parent directory does not exist, create it.
    if !db_dir.exists() {
        fs::create_dir_all(db_dir).unwrap();
    }

    // Create the database file if it does not already exist.
    if !(Path::new(&db_path)).exists(){
        fs::File::create(db_path).unwrap();
        let _ = run_migrator(db_name).await;
    }
}

/// Check whether the database file exists.
pub fn db_file_exists(db_name: String) -> bool {
    let db_path = get_db_path(&db_name);
    Path::new(&db_path).exists()
}

/// Get the path as a string where the database file should be located.
pub fn get_db_path(db_name: &str) -> String {
    let home_dir = dirs::home_dir().unwrap();
    home_dir.to_str().unwrap().to_string() + "/.config/TreeDB/" + &db_name + ".sqlite"
}

/// Deletes the sql file with the given name if it exists
#[tauri::command]
pub fn delete_db_file(db_name: String) {
    if db_file_exists(db_name.clone()) {
        let db_path = get_db_path(&db_name);
        let db_path = Path::new(&db_path);
        let _ = fs::remove_file(db_path).unwrap();
    }
}

/// Clones the db with the given name, the new db will have the clone name
#[tauri::command]
pub async fn clone_db_file(db_name: String, clone_name: String) -> Result<(), ItemDBError>{
    if !db_file_exists(clone_name.clone()){
        create_db_file(clone_name.clone()).await;
        let db_path = get_db_path(&db_name);
        let clone_path = get_db_path(&clone_name);
        fs::copy(db_path, clone_path)?;
    }
    Ok(())
}

/// Returns a vector list of all the filenames within the /.config/TreeDB/ directory
// This function has only been tested on windows.
#[tauri::command]
pub fn get_db_filenames()-> Vec<String>{
    let curr_dir = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/.config/TreeDB/";
    let paths = fs::read_dir(&curr_dir).unwrap();
    let mut filenames:Vec<String> = vec![];
    let re = Regex::new(r"([^\\/]+)\.sqlite$").unwrap();

    for path in paths{
        let path_str = path.unwrap().path().display().to_string();

        let cap = re.captures(&path_str);
        if cap.is_some(){
            let filename = cap.unwrap().get(1);
            if filename.is_some(){
                filenames.push(filename.unwrap().as_str().to_owned());
            }
        }
    }
    return filenames;
}

/// Renames a database with the given name to the new name, if exists a database with the new name, nothing happens.
#[tauri::command]
pub fn rename_db(db_name: String, new_name: String){
    if !(db_name == new_name){
        let old_db_path_str = get_db_path(&db_name);
        let new_db_path_str = get_db_path(&new_name);
        let from = Path::new(&old_db_path_str);
        let to = Path::new(&new_db_path_str);
        let _ = fs::rename(from, to);
    }
}

/// Returns a string path to a backup database file, the db file may not exist.
pub fn get_backup_db_path(db_name: &str)-> String{
    // get the directory path as a string to backups folder
    let backup_dir = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/.config/TreeDB/backups/";
    return backup_dir + db_name + ".sqlite";
}

/// Creates a backup of the given filename in the backups folder. The file generated has the date created appended to it as an identifier
#[tauri::command]
pub async fn backup_db(db_name: String, backup_name: String)-> bool{
    // check if the path exists to the file and if not, then create a file and then a clone using the db_name and backup_filename
    let backup_path_str = get_backup_db_path(&backup_name);
    let backup_path = Path::new(&backup_path_str);
    let mut res = false;
    if !backup_path.exists(){
        res = true;
        let db_path_str = get_db_path(&db_name);
        let db_path = Path::new(&db_path_str);
        fs::File::create(backup_path_str.clone()).unwrap();
        let _ = fs::copy(db_path, backup_path);
    }
    return res;
}

#[tauri::command]
pub fn restore_db(backup_name: String, new_name: String) -> bool{
    let db_path_str = get_db_path(&new_name);
    let db_path = Path::new(&db_path_str);
    let mut res = false;
    if !db_path.exists(){
        res = true;
        fs::File::create(db_path_str.clone()).unwrap();
        let backup_path_str = get_backup_db_path(&backup_name);
        let backup_path = Path::new(&backup_path_str);

        let _ = fs::copy(backup_path, db_path);
    }
    return res;
    
}

/// Gets the filenames of all backup files
#[tauri::command]
pub fn get_backup_filenames()-> Vec<String>{
    let curr_dir = dirs::home_dir().unwrap().to_str().unwrap().to_string() + "/.config/TreeDB/backups/";
    let paths = fs::read_dir(&curr_dir).unwrap();
    let mut filenames:Vec<String> = vec![];
    let re = Regex::new(r"([^\\/]+)\.sqlite$").unwrap();

    for path in paths{
        let path_str = path.unwrap().path().display().to_string();

        let cap = re.captures(&path_str);
        if cap.is_some(){
            let filename = cap.unwrap().get(1);
            if filename.is_some(){
                filenames.push(filename.unwrap().as_str().to_owned());
            }
        }
    }
    return filenames;
}

/// Renames a database with the given name to the new name, if exists a database with the new name, nothing happens.
#[tauri::command]
pub fn rename_backup(backup_name: String, new_name: String){
    if !(backup_name == new_name){
        let old_db_path_str = get_backup_db_path(&backup_name);
        let new_db_path_str = get_backup_db_path(&new_name);
        let from = Path::new(&old_db_path_str);
        let to = Path::new(&new_db_path_str);
        let _ = fs::rename(from, to);
    }
}

/// Deletes the sql file with the given name if it exists
#[tauri::command]
pub fn delete_backup_file(backup_name: String) {
    let backup_path = get_backup_db_path(&backup_name);
    let backup_path = Path::new(&backup_path);
    if backup_path.exists() {
        let _ = fs::remove_file(backup_path).unwrap();
    }
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
        create_db_file("test_database".to_string()).await;
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
            let _res = item::Entity::insert(item).exec(&db).await?;
        }
        let mut item = item::ActiveModel{..Default::default()};
        item.name = ActiveValue::Set(format!("itemy-55"));
        let res = item::Entity::insert(item).exec(&db).await?;

        assert_eq!(res.last_insert_id, 7);

        for i in 0..=5 {
            let mut tag = tag::ActiveModel{..Default::default()};
            tag.name = ActiveValue::Set(format!("taggy-{i}"));
            let _res = tag::Entity::insert(tag).exec(&db).await?;
        }
        let mut tag = tag::ActiveModel{..Default::default()};
        tag.name = ActiveValue::Set(format!("taggy-77"));
        let res = tag::Entity::insert(tag).exec(&db).await?;
        
        assert_eq!(res.last_insert_id, 7);
  
        
        // test junction
        for i in 1..=5{
            for j in 1..=5{
                let item_tag = item_tag::ActiveModel::from_json(json!({
                    "item_id": i,
                    "tag_id": j
                }))?;
                let _res = item_tag::Entity::insert(item_tag).exec(&db).await?;
            }
        }
        let item_tag = item_tag::ActiveModel::from_json(json!({
            "item_id": 6,
            "tag_id": 6
        }))?;
        let res = item_tag::Entity::insert(item_tag).exec(&db).await?;
        assert_eq!(res.last_insert_id, (6,6));
        Ok(())
    }
}

