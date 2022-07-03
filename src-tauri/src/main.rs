#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate rusqlite;

mod db;

use db::*;
use db::models::*;
use log::{info, error};
use simple_logger::SimpleLogger;

#[tauri::command]
fn get_exercises(db_state: tauri::State<DatabaseState>) -> Vec<Exercise> {
    info!("Invoking get_exercises command");
    let db = db_state.get_db();
    return db.find_exercises();
}

#[tauri::command]
fn get_exercise(db_state: tauri::State<DatabaseState>, id: String) -> Exercise {
    info!("Invoking get_exercise command with parameters: {{ id: {} }}", id);
    let db = db_state.get_db();
    return db.find_exercise(id);
}

#[tauri::command]
fn add_exercise(db_state: tauri::State<DatabaseState>, exercise: ExercisePatch) {
    info!("Invoking add_exercise command with parameters: {{ exercise: {:?} }}", exercise);
    let db = db_state.get_db();
    db.add_exercise(exercise);
}

#[tauri::command]
fn update_exercise(db_state: tauri::State<DatabaseState>, id: String, exercise: ExercisePatch) {
    info!("Invoking update_exercise command with parameters: {{ id: {}, exercise: {:?} }}", id, exercise);
    let db = db_state.get_db();
    db.update_exercise(id, exercise);
}

fn main() {
    SimpleLogger::new().init().unwrap();
    info!("Running application");
    let result = tauri::Builder::default()
        .manage(DatabaseState::new(Database::new()))
        .invoke_handler(tauri::generate_handler![get_exercises, get_exercise, add_exercise, update_exercise])
        .run(tauri::generate_context!());
    match result {
        Ok(_) => (),
        Err(err) => error!("Application run failed: {}", err),
    }
}
