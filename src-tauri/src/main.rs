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
use tauri::Manager;

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
    return db.find_exercise(&id);
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
    db.update_exercise(&id, exercise);
}

#[tauri::command]
fn delete_exercise(db_state: tauri::State<DatabaseState>, id: String) {
    info!("Invoking delete_exercise command with parameters: {{ id: {} }}", id);
    let db = db_state.get_db();
    db.delete_exercise(&id);
}

#[tauri::command]
fn get_trainings(db_state: tauri::State<DatabaseState>) -> Vec<Training> {
    info!("Invoking get_trainings command");
    let db = db_state.get_db();
    return db.find_trainings(); 
}

#[tauri::command]
fn get_training(db_state: tauri::State<DatabaseState>, id: String) -> Training {
    info!("Invoking get_training command with parameters: {{ id: {} }}", id);
    let db = db_state.get_db();
    return db.find_training(&id);
}

#[tauri::command]
fn add_training(db_state: tauri::State<DatabaseState>, training: TrainingPatch) {
    info!("Invoking add_training command with parameters: {{ training: {:?} }}", training);
    let db = db_state.get_db();
    db.add_training(training);
}

#[tauri::command]
fn update_training(db_state: tauri::State<DatabaseState>, id: String, training: TrainingPatch) {
    info!("Invoking update_training command with parameters: {{ id: {}, exercise: {:?} }}", id, training);
    let db = db_state.get_db();
    db.update_training(&id, training); 
}

#[tauri::command]
fn delete_training(db_state: tauri::State<DatabaseState>, id: String) {
    info!("Invoking delete_training command with parameters: {{ id: {} }}", id);
    let db = db_state.get_db();
    db.delete_training(&id);
}

#[tauri::command]
fn get_exercise_details(db_state: tauri::State<DatabaseState>, exercise_id: String) -> Vec<ExerciseDetails> {
    info!("Invoking get_exercise_details with parameters: {{ id: {} }}", exercise_id);
    let db = db_state.get_db();
    return db.find_exercise_details(exercise_id);
}

fn main() {
    SimpleLogger::new().init().unwrap();
    info!("Running application");
    let result = tauri::Builder::default()
        .setup(|app| {
            let path_resolver = app.path_resolver();
            let app_dir = path_resolver.app_dir().unwrap().into_os_string().into_string().unwrap();
            app.manage(DatabaseState::new(Database::new(app_dir)));
            return Ok(());
        })
        .invoke_handler(tauri::generate_handler![
            get_exercises, get_exercise, add_exercise, update_exercise, delete_exercise,
            get_trainings, get_training, add_training, update_training, delete_training,
            get_exercise_details
            ])
        .run(tauri::generate_context!());
    match result {
        Ok(_) => (),
        Err(err) => error!("Application run failed: {}", err),
    }
}
