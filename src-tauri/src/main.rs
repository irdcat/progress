#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
use db::*;
use db::models::*;

#[tauri::command]
fn create_test_data(db_state: tauri::State<DatabaseState>, name: &str) -> Vec<Test> {
  let db = db_state.get_db();  
  db.create_test(name);
  return db.get_test();
}

fn main() {
  tauri::Builder::default()
    .manage(DatabaseState::new(Database::new()))
    .invoke_handler(tauri::generate_handler![create_test_data])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
