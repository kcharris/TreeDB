// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod db;
pub mod migrator;
pub mod entities;
pub mod errors;

use futures::executor::block_on;
use crate::db::db_util::*;
use crate::db::tag_db_util::*;
use crate::db::item_tag_db_util::*;
use crate::db::item_db_util::*;

use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn open_file_explorer(dir_address: &str) {
    println!("Opening");
    let mut os_command = "".to_owned();
    if cfg!(windows){
        os_command = "explorer".to_string();
    }
    // Unix has not been tested
    else if cfg!(unix){
        if cfg!(target_os = "macos"){
            os_command= "open".to_string();
        }
        // This assumes a linux os
        else{
            os_command="xdg-open".to_string();
        }
    }
    Command::new(os_command)
        .arg(dir_address)
        .spawn()
        .unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_items_by_parent_id, get_item_by_id, add_item, delete_item, update_item, open_file_explorer])
        .setup(|_app|{
            init();
            if let Err(err) = block_on(run_migrator()) {
                panic!("{}", err);
            }
            Ok(())
            }
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
