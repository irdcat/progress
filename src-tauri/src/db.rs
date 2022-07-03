pub mod models;

use std::{sync::{Arc, Mutex}, ops::Deref};
use log::{trace, info, error};
use rusqlite::{Connection, OpenFlags};
use uuid::Uuid;

use models::*;

const DEFAULT_DATABASE_FILE: &str = "test.db";

pub struct Database {
    connection_arc_mutex: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(path: String) -> Self {
        return Database { connection_arc_mutex: Arc::new(Mutex::new(Database::establish_connection(path))) };
    }

    fn establish_connection(mut path: String) -> Connection {
        path.push_str(if path.contains("\\") {"\\"} else {"/"});
        path.push_str(DEFAULT_DATABASE_FILE);
        match Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_WRITE) {
            Ok(c) => return c,
            Err(_) => {
                let c = Connection::open(&path).unwrap();
                Database::initialize_db_model(&c);
                return c
            },
        };
    }

    fn initialize_db_model(c: &Connection) {
        const CREATE_EXERCISES_TABLE_SQL_STATEMENT: &str = "
            CREATE TABLE exercises (
                id          VARCHAR NOT NULL PRIMARY KEY,
                name        VARCHAR NOT NULL,
                description VARCHAR,
                bodyweight  BOOLEAN
            )";

        let create_table_statements = [CREATE_EXERCISES_TABLE_SQL_STATEMENT];

        for statement in create_table_statements {
            trace!("Executing SQL statement {}", statement);
            c.execute(statement, []).unwrap();
        }
    }
}

pub trait ExercisesOperations {
    fn find_exercises(&self) -> Vec<Exercise>;
    fn find_exercise(&self, id: String) -> Exercise;
    fn add_exercise(&self, exercise: ExercisePatch);
    fn update_exercise(&self, id: String, exercise: ExercisePatch);
}

impl ExercisesOperations for Database {
    fn find_exercises(&self) -> Vec<Exercise> {
        const FIND_EXERCISES_SQL_STATEMENT: &str = "SELECT * FROM exercises";
        
        let connection_guard = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_guard.deref();

        let mut statement = connection.prepare(FIND_EXERCISES_SQL_STATEMENT).unwrap();
        let exercise_iterator = statement.query_map([], |row| {
            Ok(Exercise{
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                description: row.get(2).unwrap(),
                bodyweight: row.get(3).unwrap()
            })
        }).unwrap();

        let mut result = Vec::new();
        for exercise in exercise_iterator {
            result.push(exercise.unwrap());
        }
        return result;
    }

    fn find_exercise(&self, exercise_id: String) -> Exercise {
        const FIND_EXERCISE_SQL_STATEMENT: &str = "SELECT * FROM exercises WHERE id = ?";
        
        let connection_guard = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_guard.deref();

        let mut statement = connection.prepare(FIND_EXERCISE_SQL_STATEMENT).unwrap();
        let mut exercise_iterator = statement.query_map([exercise_id], |row| {
            Ok(Exercise{
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                description: row.get(2).unwrap(),
                bodyweight: row.get(3).unwrap()
            })
        }).unwrap();

        return exercise_iterator.next().unwrap().unwrap();
    }

    fn add_exercise(&self, exercise: ExercisePatch) {
        const ADD_EXERCISE_SQL_STATEMENT: &str = "INSERT INTO exercises (id, name, description, bodyweight) VALUES (?1, ?2, ?3, ?4)";

        let connection_guard = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_guard.deref();

        let uuid = Uuid::new_v4().as_hyphenated().to_string();
        let mut statement = connection.prepare(ADD_EXERCISE_SQL_STATEMENT).unwrap();
        match statement.execute(params![uuid, exercise.name, exercise.description, exercise.bodyweight]) {
            Ok(_) => info!("Insert successful"),
            Err(err) => {
                error!("Insert failed: {}", err);
                error!("Statement: {}", statement.expanded_sql().unwrap());
            },
        }
    }

    fn update_exercise(&self, exercise_id: String, exercise: ExercisePatch) {
        const UPDATE_EXERCISE_SQL_STATEMENT: &str = "UPDATE exercises SET name = ?1, description = ?2, bodyweight = ?3 WHERE id = ?4";

        let connection_guard = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_guard.deref();

        let mut statement = connection.prepare(UPDATE_EXERCISE_SQL_STATEMENT).unwrap();
        match statement.execute(params![exercise.name, exercise.description, exercise.bodyweight, exercise_id]) {
            Ok(_) => info!("Update successful"),
            Err(err) => {
                error!("Update failed: {}", err);
                error!("Statement: {}", statement.expanded_sql().unwrap());
            },
        }
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
