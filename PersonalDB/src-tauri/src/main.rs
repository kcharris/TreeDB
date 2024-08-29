// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod db;
pub mod migrator;
pub mod entities;
pub mod errors;

use futures::executor::block_on;
use crate::db::db_util::*;
use crate::db::item_db_util::*;
use crate::db::tag_db_util::*;
use crate::db::item_tag_db_util::*;
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
        .invoke_handler(tauri::generate_handler![find_items_by_parent_id, get_item_by_id, add_item, delete_item,
            update_item, open_file_explorer, update_on_start_db, create_db_file,
            delete_db_file, clone_db_file, get_db_filenames, get_db_name, rename_db,
            backup_db, restore_db, get_backup_filenames, rename_backup, delete_backup_file, get_tags, update_tag,
            delete_tag, add_tag, add_item_tag, get_tags_by_item_id, get_items_by_tag_id, delete_item_tag])
        .setup(|_app|{
            if let Err(err) = block_on(init()) {
                panic!("{}", err);
            }
            Ok(())
            }
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
