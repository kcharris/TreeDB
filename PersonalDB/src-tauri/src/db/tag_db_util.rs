


use std::result::Result;
use crate::errors::ItemDBError;


use crate::entities::*;
use sea_orm_migration::prelude::*;
use sea_orm::{ActiveValue, ActiveModelTrait, EntityTrait, ModelTrait};
use crate::db::db_util::*;



#[tauri::command]
pub async fn add_tag(db_name: String, name: String) -> Result<i32, ItemDBError> {
  let db = get_db_conn(&db_name).await?;
  let mut tag = tag::ActiveModel{..Default::default()};
  tag.name = ActiveValue::set(name);

  let res = tag::Entity::insert(tag).exec(&db).await?;
  Ok(res.last_insert_id)
}

#[tauri::command]
pub async fn get_tags(db_name: String) -> Result<String, ItemDBError>{
    let db = get_db_conn(&db_name).await?;
    let _res: String;
    let tags: serde_json::Value;

    tags = sea_orm::JsonValue::Array(tag::Entity::find().into_json().all(&db).await?);
    
    let res = tags;
    Ok(res.to_string())
}

#[tauri::command]
pub async fn update_tag(db_name: String, payload: String) -> Result<(), ItemDBError>{
  let db = get_db_conn(&db_name).await?;
  let json_tag: serde_json::Value = serde_json::from_str(&payload).unwrap();
  let mut tag = tag::ActiveModel{..Default::default()};
  tag.set_from_json(json_tag.clone())?;
  let n: i32 = json_tag["id"].as_i64().unwrap().try_into()?;
  tag.id = ActiveValue::Set(n);
  
  let _res = tag.save(&db).await?;
  Ok(())
}

#[tauri::command]
pub async fn delete_tag(db_name: String, id: i32) -> Result<u64, ItemDBError>{
    let db = get_db_conn(&db_name).await?;
    let tag = tag::Entity::find_by_id(id).one(&db).await?.unwrap();
    let res = tag.delete(&db).await?;
    Ok(res.rows_affected)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct Tag {
      id: i32,
      name: String
    }

    // test with "$ cargo test -- --test-threads=1 ", this is because of issues with async functions sharing resources
    // Sets up a test database to avoid over-writing original
    pub async fn setup() -> Result<(), ItemDBError> {
        delete_db_file("test_database");
        create_db_file("test_database");
        run_migrator("test_database").await?;

        return Ok(());
    }

    #[async_std::test]
    pub async fn test_async_tag() -> Result<(), ItemDBError> {
        setup().await?;
        let db_name = "test_database".to_owned();

        // test insert
        let name1 = "tag1".to_string();
        let name2 = "tag2".to_string();

        let res = add_tag(db_name.clone(), name1).await?;
        let res2 = add_tag(db_name.clone(), name2).await?;

        assert_eq!(res, 1, "tag insert failed");
        assert_eq!(res2, 2, "tag insert failed");
        
    
        // test get
        let tags_str = get_tags(db_name.clone()).await?;
        let tags:Vec<Tag> = serde_json::from_str(&tags_str).unwrap();
        let name = &tags[0].name;

        assert!(tags[0].name == "tag1".to_owned() || tags[0].name == "tag2".to_owned(), "tag get failed");
        assert_eq!(tags.len(), 2);


        // test update
        let json_str = r#"{
          "id": 1,
          "name": "taggy"
        }"#.to_owned();

        update_tag(db_name.clone(), json_str).await?;
        let db = get_db_conn(&db_name).await?;
        let tag = tag::Entity::find_by_id(1).one(&db).await?.unwrap();
        assert_eq!(tag.name, "taggy".to_owned(), "tag update failed");
        
        // test delete
        let res = delete_tag(db_name, 1).await?;

        assert_eq!(res, 1, "delete failed");
        Ok(())
    }
}