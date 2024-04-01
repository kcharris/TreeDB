// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod db;
pub mod migrator;
pub mod entities;

use db::db_init::establish_sql_connection;
use futures::executor::block_on;
use sea_orm::{Database, DbErr};
use crate::db::db_init;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

async fn run()  -> Result<(), DbErr> {
    establish_sql_connection().await?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
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
