// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod db;
pub mod migrator;
pub mod entities;

use db::db_init::run_migrator;
use futures::executor::block_on;
use sea_orm::{Database, DbErr};
use crate::db::db_init;
use crate::db::db_util::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(msg: &str) -> String {
    println!("Hello from {}", msg);
    "Greet".to_owned()
}

async fn run()  -> Result<(), Error> {
    run_migrator().await?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_items_by_parent_id, add_item, greet, delete_item, update_item])
        .setup(|_app|{
            db_init::init();
            if let Err(err) = block_on(run()) {
                panic!("{}", err);
            }
            Ok(())
            }
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
