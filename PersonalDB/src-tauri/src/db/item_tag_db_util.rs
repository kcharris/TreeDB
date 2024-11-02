use crate::errors::ItemDBError;
use crate::entities::*;
use sea_orm_migration::prelude::*;
use sea_orm::{ActiveValue, EntityTrait, ModelTrait};
use crate::db::db_util::*;

/// Adds an item tag based on an item id and tag id
#[tauri::command]
pub async fn add_item_tag(db_name: String, item_id:i32, tag_id:i32) -> Result<(i32, i32), ItemDBError> {
  let db = get_db_conn(&db_name).await?;
  let mut item_tag = item_tag::ActiveModel{..Default::default()};
  item_tag.item_id = ActiveValue::set(item_id);
  item_tag.tag_id = ActiveValue::set(tag_id);

  let res = item_tag::Entity::insert(item_tag).exec(&db).await?;
  Ok(res.last_insert_id)
}

/// Gets a JSON string containing a list of tags based on a item id
#[tauri::command]
pub async fn get_tags_by_item_id(db_name: String, id:i32) -> Result<String, ItemDBError>{
    let db = get_db_conn(&db_name).await?;
    let item = item::Entity::find_by_id(id).one(&db).await?.unwrap();
    
    let tags = sea_orm::JsonValue::Array(item.find_related(tag::Entity).into_json().all(&db).await?);
    Ok(tags.to_string())
}

/// Gets a JSON string containing a list of items based on a tag id
#[tauri::command]
pub async fn get_items_by_tag_id(db_name: String, id:i32) -> Result<String, ItemDBError>{
    let db = get_db_conn(&db_name).await?;
    let tag = tag::Entity::find_by_id(id).one(&db).await?.unwrap();

    let items = sea_orm::JsonValue::Array(tag.find_related(item::Entity).into_json().all(&db).await?);
    Ok(items.to_string())
}

/// Deletes an item tag using its primary key
#[tauri::command]
pub async fn delete_item_tag(db_name: String, item_id: i32, tag_id:i32) -> Result<u64, ItemDBError>{
    let db = get_db_conn(&db_name).await?;
    let item_tag = item_tag::Entity::find_by_id((item_id, tag_id)).one(&db).await?.unwrap();
    let res = item_tag.delete(&db).await?;
    Ok(res.rows_affected)
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::tag_db_util::add_tag;
    use crate::add_item;

    // test with "$ cargo test -- --test-threads=1 ", this is because of issues with async functions sharing resources
    // Sets up a test database to avoid over-writing original
    pub async fn setup() -> Result<(), ItemDBError> {
        delete_db_file("test_database".to_owned());
        create_db_file("test_database".to_owned());
        run_migrator("test_database".to_owned()).await?;

        return Ok(());
    }

    #[async_std::test]
    pub async fn test_async_item_tag() -> Result<(), ItemDBError> {
        setup().await?;
        let db_name = "test_database".to_owned();

        // test insert

        for i in 0..6{
            add_tag(db_name.clone(), "tag".to_owned()).await?;
        }

        let json_str = r#"{
            "name": "item"
        }"#.to_owned();
        for i in 0..6{
            add_item(db_name.clone(), json_str.clone()).await?;
        }
        let res = add_item(db_name.clone(), json_str.clone()).await?;
        //assert_eq!(res, Some(7), "item insert failed");

        for i in 1..5{
            for j in 1..5{
                add_item_tag(db_name.clone(), i, j).await?;
            }
        }
        let res = add_item_tag(db_name.clone(), 5, 5).await?;
        assert_eq!(res, (5, 5), "item tag 1 insert failed");
        
    
        // test get
        let items = get_items_by_tag_id(db_name.clone(), 3).await?;
        let items:Vec<serde_json::Value> = serde_json::from_str(&items).unwrap();

        assert_eq!(items.len(), 4, "get items from tag id failed");

        let tags = get_tags_by_item_id(db_name.clone(), 3).await?;
        let tags:Vec<serde_json::Value> = serde_json::from_str(&tags).unwrap();

        assert_eq!(tags.len(), 4, "get tags from item id failed");


        // test delete
        let res = delete_item_tag(db_name.clone(), 5, 5).await?;
        assert_eq!(res, 1, "delete item tag failed");

        // let res = delete_item(db_name.clone(), 1).await?;
        // assert_eq!(res, 6, "delete item failed to cascade delete related item_tags");

        Ok(())
    }
}