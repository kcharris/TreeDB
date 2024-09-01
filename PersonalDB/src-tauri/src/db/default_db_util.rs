use crate::db::db_util::*;
use crate::db::item_db_util::*;
use crate::db::tag_db_util::*;
use crate::db::item_tag_db_util::*;
use sea_orm::{Database, DatabaseConnection, DbErr};
use crate::errors::ItemDBError;

// This file is for calling one function in db_util to populate a new default database with default values.
pub async fn set_default_values(db_name: String)-> Result<(), ItemDBError>{

    let _ = add_tag(db_name.clone(), "Favorite".to_string()).await?;
    let _ = add_tag(db_name.clone(), "Current".to_string()).await?;


    // create a list of json items to add items are inserted by tree levels or depths
    // depth 1
    let fun_id = add_item(db_name.clone(), r#"{
        "name": "Fun"
    }"#.to_string()).await?;

    let work_id = add_item(db_name.clone(), r#"{
        "name": "Work"
    }"#.to_string()).await?;

    let inventory_id = add_item(db_name.clone(), r#"{
        "name": "Inventory"
    }"#.to_string()).await?;

    let cooking_id = add_item(db_name.clone(), r#"{
        "name": "Cooking"
    }"#.to_string()).await?;

    let misc_id = add_item(db_name.clone(), r#"{
        "name": "Misc"
    }"#.to_string()).await?;

    

    // depth 2
    // push to fun
    let show_id = add_item(db_name.clone(), format!(r#"{{
        "name": "Show",
        "parent_id": {}
    }}"#, fun_id).to_string()).await?;

    let movie_id = add_item(db_name.clone(), format!(r#"{{
        "name": "Movie",
        "parent_id": {}
    }}"#, fun_id).to_string()).await?;

    let book_id = add_item(db_name.clone(), format!(r#"{{
        "name": "Book",
        "parent_id": {}
    }}"#, fun_id).to_string()).await?;


    // push to work
    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "Resume",
        "parent_id": {}
    }}"#, work_id).to_string()).await?;

    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "Applied",
        "parent_id": {}
    }}"#, work_id).to_string()).await?;


    // push to invetory
    //cleaning
    let cleaning_id = add_item(db_name.clone(), format!(r#"{{
        "name": "Cleaning",
        "parent_id": {}
    }}"#, inventory_id).to_string()).await?;

    //finances
    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "Finances",
        "parent_id": {}
    }}"#, inventory_id).to_string()).await?;


    // push to cooking
    let juice_id = add_item(db_name.clone(), format!(r#"{{
        "name": "Juice Rankings",
        "parent_id": {},
        "description": "Ranking for Juices that have been diluted with water. ~2oz juice to 14oz water"
    }}"#, cooking_id).to_string()).await?;

    let recipe_id = add_item(db_name.clone(), format!(r#"{{
        "name": "Recipes",
        "parent_id": {}
    }}"#, cooking_id).to_string()).await?;



    // depth 3?
    // movies
    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "John Candlestring",
        "parent_id": {}
    }}"#, movie_id).to_string()).await?;

    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "Fury Highway",
        "parent_id": {}
    }}"#, show_id).to_string()).await?;

    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "Pix R's Down",
        "parent_id": {}
    }}"#, movie_id).to_string()).await?;


    // Shows
    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "The Sparrow Apartment",
        "parent_id": {}
    }}"#, show_id).to_string()).await?;

    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "Antigravity Falls",
        "parent_id": {}
    }}"#, show_id).to_string()).await?;

    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "The Large Explosion Hypothesis",
        "parent_id": {}
    }}"#, show_id).to_string()).await?;


    // books
    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "The Wheel of Clock",
        "parent_id": {}
    }}"#, book_id).to_string()).await?;

    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "Lord of the Necklaces",
        "parent_id": {}
    }}"#, book_id).to_string()).await?;


    // Juices
    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "Organic Apple",
        "parent_id": {},
        "priority": 95
    }}"#, juice_id).to_string()).await?;

    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "Organic Grape",
        "parent_id": {},
        "priority": 100
    }}"#, juice_id).to_string()).await?;

    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "Organic Tart Cherry",
        "parent_id": {},
        "priority": 100
    }}"#, juice_id).to_string()).await?;

    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "Organic Cranberry",
        "parent_id": {},
        "priority": 20
    }}"#, juice_id).to_string()).await?;

    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "Organic Blueberry",
        "parent_id": {},
        "priority": 30
    }}"#, juice_id).to_string()).await?;


    // cleaning
    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "House",
        "parent_id": {}
    }}"#, cleaning_id).to_string()).await?;

    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "Bathroom",
        "parent_id": {}
    }}"#, cleaning_id).to_string()).await?;

    let _ = add_item(db_name.clone(), format!(r#"{{
        "name": "Yard",
        "parent_id": {}
    }}"#, cleaning_id).to_string()).await?;

    return Ok(())
}