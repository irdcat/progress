#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate diesel;

mod db;
mod payloads;

use db::*;
use db::models::*;
use payloads::*;
use log::info;
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
fn add_exercise(db_state: tauri::State<DatabaseState>, exercise: ExercisePayload) {
    info!("Invoking add_exercise command with parameters: {{ exercise: {:?} }}", exercise);
    let new_exercise = NewExercise{
        name: match &exercise.name {
            Some(x) => x.as_str(),
            None => panic!("Name required for new exercise"),
        },
        description: match &exercise.description {
            Some(x) => Some(x.as_str()),
            None => None,
        },
        bodyweight: match exercise.bodyweight {
            Some(x) => x,
            None => false,
        }
    };
    let db = db_state.get_db();
    db.add_exercise(new_exercise);
}

#[tauri::command]
fn update_exercise(db_state: tauri::State<DatabaseState>, id: String, exercise: ExercisePayload) {
    info!("Invoking update_exercise command with parameters: {{ id: {}, exercise: {:?} }}", id, exercise);
    let update_exercise = UpdateExercise {
        name: match &exercise.name {
            Some(x) => Some(x.as_str()),
            None => None,
        },
        description: match &exercise.description {
            Some(x) => Some(x.as_str()),
            None => None,
        },
        bodyweight: exercise.bodyweight,
    };
    let db = db_state.get_db();
    db.update_exercise(id, update_exercise);
}

fn main() {
    SimpleLogger::new().init().unwrap();
    info!("Running application");
    tauri::Builder::default()
        .manage(DatabaseState::new(Database::new()))
        .invoke_handler(tauri::generate_handler![get_exercises, get_exercise, add_exercise, update_exercise])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
