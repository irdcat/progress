pub mod models;
pub mod schema;

use std::{sync::{Arc, Mutex}, ops::Deref};
use models::*;
use schema::exercises::dsl::*;

use diesel::prelude::*;
use uuid::Uuid;

const DEFAULT_DATABASE_URL: &str = "test.db";

pub struct Database {
    connection_arc_mutex: Arc<Mutex<SqliteConnection>>,
}

impl Database {
    pub fn new() -> Self {
        return Database { 
            connection_arc_mutex: Arc::new(
                Mutex::new(
                    Database::establish_connection()
                )) 
        };
    }

    fn establish_connection() -> SqliteConnection {
        return SqliteConnection::establish(DEFAULT_DATABASE_URL)
            .unwrap_or_else(|_| panic!("Error connecting to {}", DEFAULT_DATABASE_URL));
    }
}

pub trait ExercisesOperations {
    fn find_exercises(&self) -> Vec<Exercise>;
    fn find_exercise(&self, id: String) -> Exercise;
    fn add_exercise(&self, exercise: NewExercise);
    fn update_exercise(&self, id: String, exercise: UpdateExercise);
}

impl ExercisesOperations for Database {
    fn find_exercises(&self) -> Vec<Exercise> {
        let connection_arc_mutex = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_arc_mutex.deref();

        return exercises.load::<Exercise>(connection)
            .expect("Failed to load exercises");
    }

    fn find_exercise(&self, exercise_id: String) -> Exercise {
        let connection_arc_mutex = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_arc_mutex.deref();
        
        return exercises.find(&exercise_id)
            .first(connection)
            .expect(format!("Could not load exercise with id: {}", exercise_id).as_str());
    }

    fn add_exercise(&self, exercise: NewExercise) {
        let connection_arc_mutex = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_arc_mutex.deref();
        
        let uuid = Uuid::new_v4().hyphenated().to_string();

        diesel::insert_into(exercises)
            .values((id.eq(uuid), exercise))
            .execute(connection)
            .expect("Could not add new exercise");
    }

    fn update_exercise(&self, exercise_id: String, exercise: UpdateExercise) {
        let connection_arc_mutex = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_arc_mutex.deref();

        diesel::update(exercises)
            .set(&exercise)
            .filter(id.eq(&exercise_id))
            .execute(connection)
            .expect(format!("Could not update exercise with id {}", exercise_id).as_str());
    }
}

pub struct DatabaseState(Database);

impl DatabaseState {
    pub fn new(db: Database) -> Self {
        return DatabaseState(db);
    }

    pub fn get_db(&self) -> &Database {
        return &self.0;
    }
}
