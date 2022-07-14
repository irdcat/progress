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
        const CREATE_TRAININGS_TABLE_SQL_STATEMENT: &str = "
            CREATE TABLE trainings (
                id          VARCHAR NOT NULL PRIMARY KEY,
                date        VARCHAR NOT NULL
            )";
        const CREATE_TRAINING_ENTRIES_TABLE_SQL_STATEMENT: &str = "
            CREATE TABLE training_entries (
                id          VARCHAR NOT NULL PRIMARY KEY,
                exercise_id VARCHAR NOT NULL,
                training_id VARCHAR NOT NULL,
                FOREIGN KEY (training_id) 
                    REFERENCES trainings(id)
            )";
        const CREATE_TRAINING_SETS_TABLE_SQL_STATEMENT: &str = "
            CREATE TABLE training_sets (
                id          VARCHAR NOT NULL PRIMARY KEY,
                repetitions INTEGER NOT NULL,
                weight      REAL NOT NULL,
                entry_id    VARCHAR NOT NULL,
                FOREIGN KEY (entry_id) 
                    REFERENCES training_entries (id)
            )";

        let create_table_statements = [
            CREATE_EXERCISES_TABLE_SQL_STATEMENT,
            CREATE_TRAININGS_TABLE_SQL_STATEMENT,
            CREATE_TRAINING_ENTRIES_TABLE_SQL_STATEMENT,
            CREATE_TRAINING_SETS_TABLE_SQL_STATEMENT];

        for statement in create_table_statements {
            trace!("Executing SQL statement {}", statement);
            c.execute(statement, []).unwrap();
        }
    }
}

pub trait ExercisesOperations {
    fn find_exercises(&self) -> Vec<Exercise>;
    fn find_exercise(&self, id: &String) -> Exercise;
    fn add_exercise(&self, exercise: ExercisePatch);
    fn update_exercise(&self, id: &String, exercise: ExercisePatch);
}

impl ExercisesOperations for Database {
    fn find_exercises(&self) -> Vec<Exercise> {
        const FIND_EXERCISES_SQL_STATEMENT: &str = "SELECT * FROM exercises";
        
        let connection_guard = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_guard.deref();

        let mut statement = connection.prepare(FIND_EXERCISES_SQL_STATEMENT).unwrap();
        let exercise_iterator = statement.query_map([], |row| {
            return Ok(Exercise{
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                description: row.get(2).unwrap(),
                bodyweight: row.get(3).unwrap()
            });
        }).unwrap();

        let mut result = Vec::new();
        for exercise in exercise_iterator {
            result.push(exercise.unwrap());
        }
        return result;
    }

    fn find_exercise(&self, exercise_id: &String) -> Exercise {
        const FIND_EXERCISE_SQL_STATEMENT: &str = "SELECT * FROM exercises WHERE id = ?";
        
        let connection_guard = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_guard.deref();

        let mut statement = connection.prepare(FIND_EXERCISE_SQL_STATEMENT).unwrap();
        let result = statement.query_row([exercise_id], |row| {
            return Ok(Exercise{
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
                description: row.get(2).unwrap(),
                bodyweight: row.get(3).unwrap()
            });
        });
        
        return result.unwrap();
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

    fn update_exercise(&self, exercise_id: &String, exercise: ExercisePatch) {
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

pub trait TrainingSetOperations {
    fn find_set(&self, connection: &Connection, id: &String) -> TrainingSet;
    fn find_sets_by_entry_id(&self, connection: &Connection, entry_id: &String) -> Vec<TrainingSet>;
    fn add_set(&self, connection: &Connection, entry_id: &String, set: TrainingSet);
    fn update_set(&self, connection: &Connection, set: TrainingSet);
}

impl TrainingSetOperations for Database {
    fn find_set(&self, connection: &Connection, id: &String) -> TrainingSet {
        const FIND_SET_SQL_STATEMENT: &str = "SELECT id, repetitions, weight FROM training_sets WHERE id = ?1";

        let mut statement = connection.prepare(FIND_SET_SQL_STATEMENT).unwrap();
        let result = statement.query_row([id], |row| {
            Ok(TrainingSet{
                id: row.get(0).unwrap(),
                repetitions: row.get(1).unwrap(),
                weight: row.get(2).unwrap()
            })
        });

        return result.unwrap();
    }

    fn find_sets_by_entry_id(&self, connection: &Connection, entry_id: &String) -> Vec<TrainingSet> {
        const FIND_SET_BY_ENTRY_ID_SQL_STATEMENT: &str = "SELECT id, repetitions, weight FROM training_sets WHERE entry_id = ?1";

        let mut statement = connection.prepare(FIND_SET_BY_ENTRY_ID_SQL_STATEMENT).unwrap();
        let training_set_iterator = statement.query_map([entry_id], |row| {
            Ok(TrainingSet{
                id: row.get(0).unwrap(),
                repetitions: row.get(1).unwrap(),
                weight: row.get(2).unwrap()
            })
        }).unwrap();

        let mut result = Vec::new();
        for set in training_set_iterator {
            result.push(set.unwrap());
        }
        return result;
    }

    fn add_set(&self, connection: &Connection, entry_id: &String, set: TrainingSet) {
        const ADD_SET_SQL_STATEMENT: &str = "INSERT INTO training_sets (id, repetitions, weight, entry_id) VALUES (?1, ?2, ?3, ?4)";

        let uuid = Uuid::new_v4().as_hyphenated().to_string();
        let mut statement = connection.prepare(ADD_SET_SQL_STATEMENT).unwrap();
        match statement.execute(params![uuid, set.repetitions, set.weight, entry_id]) {
            Ok(_) => info!("Insert succesful"),
            Err(err) => {
                error!("Insert failed: {}", err);
                error!("Statement: {}", statement.expanded_sql().unwrap());
            },
        }
    }

    fn update_set(&self, connection: &Connection, set: TrainingSet) {
        const UPDATE_SET_SQL_STATEMENT: &str = "UPDATE training_sets SET repetitions = ?1, weight = ?2 WHERE id = ?3";

        if set.id.is_empty() {
            error!("Id of the set to update is empty!");
            panic!("Id of the set to update is empty!");
        }

        let mut statement = connection.prepare(UPDATE_SET_SQL_STATEMENT).unwrap();
        match statement.execute(params![set.repetitions, set.weight, set.id]) {
            Ok(_) => info!("Update succesful"),
            Err(err) => {
                error!("Update failed: {}", err);
                error!("Statement: {}", statement.expanded_sql().unwrap());
            },
        }
    }
}

pub trait TrainingEntryOperations {
    fn find_entry(&self, connection: &Connection, id: &String) -> TrainingEntry;
    fn find_entries_by_training_id(&self, connection: &Connection, training_id: &String) -> Vec<TrainingEntry>;
    fn add_entry(&self, connection: &Connection, training_id: &String, entry: TrainingEntry);
    fn update_entry(&self, connection: &Connection, entry: TrainingEntry);
}

impl TrainingEntryOperations for Database {
    fn find_entry(&self, connection: &Connection, id: &String) -> TrainingEntry {
        const FIND_ENTRY_SQL_STATEMENT: &str = "SELECT id, exercise_id FROM training_entries WHERE id = ?1";

        let mut statement = connection.prepare(FIND_ENTRY_SQL_STATEMENT).unwrap();
        let result = statement.query_row([id], |row| {
            
            let id = row.get(0).unwrap();
            let exercise_id = row.get(1).unwrap();
            let sets = self.find_sets_by_entry_id(&connection, &id);

            return Ok(TrainingEntry{
                id: id,
                exercise_id: exercise_id,
                sets: sets
            });
        });

        return result.unwrap();
    }

    fn find_entries_by_training_id(&self, connection: &Connection, training_id: &String) -> Vec<TrainingEntry> {
        const FIND_ENTRY_BY_TRAINING_ID_SQL_STATEMENT: &str = "SELECT id, exercise_id FROM training_entries WHERE training_id = ?1";

        let mut statement = connection.prepare(FIND_ENTRY_BY_TRAINING_ID_SQL_STATEMENT).unwrap();
        let entry_iterator = statement.query_map([training_id], |row| {
            
            let id = row.get(0).unwrap();
            let exercise_id = row.get(1).unwrap();
            let sets = self.find_sets_by_entry_id(&connection, &id);

            return Ok(TrainingEntry{
                id: id,
                exercise_id: exercise_id,
                sets: sets
            });
        }).unwrap();

        let mut result = Vec::new();
        for entry in entry_iterator {
            result.push(entry.unwrap());
        }
        return result;
    }

    fn add_entry(&self, connection: &Connection, training_id: &String, entry: TrainingEntry) {
        const ADD_ENTRY_SQL_STATEMENT: &str = "INSERT INTO training_entries (id, exercise_id, training_id) VALUES (?1, ?2, ?3)";

        let uuid = Uuid::new_v4().as_hyphenated().to_string();
        let mut statement = connection.prepare(ADD_ENTRY_SQL_STATEMENT).unwrap();
        match statement.execute(params![uuid, entry.exercise_id, training_id]) {
            Ok(_) => {
                info!("Insert successful");
                for set in entry.sets {
                    self.add_set(&connection, &uuid, set);
                }
            },
            Err(err) => {
                error!("Insert failed: {}", err);
                error!("Statement: {}", statement.expanded_sql().unwrap());
            },
        }
    }

    fn update_entry(&self, connection: &Connection, entry: TrainingEntry) {
        const UPDATE_ENTRY_SQL_STATEMENT: &str = "UPDATE training_entries SET exercise_id = ?1 WHERE id = ?2";

        let mut statement = connection.prepare(UPDATE_ENTRY_SQL_STATEMENT).unwrap();
        match statement.execute(params![entry.exercise_id, entry.id]) {
            Ok(_) => {
                info!("Update successful");
                for set in entry.sets {
                    self.update_set(&connection, set);
                }
            },
            Err(err) => {
                error!("Insert failed: {}", err);
                error!("Statement: {}", statement.expanded_sql().unwrap());
            },
        }
    }
}

pub trait TrainingOperations {
    fn find_trainings(&self) -> Vec<Training>;
    fn find_training(&self, id: &String) -> Training;
    fn add_training(&self, training: TrainingPatch);
    fn update_training(&self, training_id: &String, training: TrainingPatch);
}

impl TrainingOperations for Database {
    fn find_trainings(&self) -> Vec<Training> {
        const FIND_TRAININGS_SQL_STATEMENT: &str = "SELECT id, date FROM trainings";

        let connection_guard = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_guard.deref();

        let mut statement = connection.prepare(FIND_TRAININGS_SQL_STATEMENT).unwrap();
        let training_iterator = statement.query_map([], |row| {
            
            let id = row.get(0).unwrap();
            let date = row.get(1).unwrap();
            let entries = self.find_entries_by_training_id(&connection, &id);

            return Ok(Training{
                id: id,
                date: date,
                entries: entries
            });
        }).unwrap();

        let mut result = Vec::new();
        for training in training_iterator {
            result.push(training.unwrap());
        }
        return result;
    }

    fn find_training(&self, id: &String) -> Training {
        const FIND_TRAINING_SQL_STATEMENT: &str = "SELECT id, date FROM trainings WHERE id = ?1";

        let connection_guard = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_guard.deref();

        let mut statement = connection.prepare(FIND_TRAINING_SQL_STATEMENT).unwrap();
        let result = statement.query_row([id], |row| {
            
            let id = row.get(0).unwrap();
            let date = row.get(1).unwrap();
            let entries = self.find_entries_by_training_id(&connection, &id);

            return Ok(Training{
                id: id,
                date: date,
                entries: entries
            });
        });

        return result.unwrap();
    }

    fn add_training(&self, training: TrainingPatch) {
        const ADD_TRAINING_SQL_STATEMENT: &str = "INSERT INTO trainings (id, date) VALUES (?1, ?2)";

        let connection_guard = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_guard.deref();

        let uuid = Uuid::new_v4().as_hyphenated().to_string();
        let mut statement = connection.prepare(ADD_TRAINING_SQL_STATEMENT).unwrap();
        match statement.execute(params![uuid, training.date]) {
            Ok(_) => {
                info!("Insert successful");
                for entry in training.entries {
                    self.add_entry(&connection, &uuid, entry);
                }
            },
            Err(err) => {
                error!("Insert failed: {}", err);
                error!("Statement: {}", statement.expanded_sql().unwrap());
            },
        }
    }

    fn update_training(&self, training_id: &String, training: TrainingPatch) {
        const UPDATE_TRAINING_SQL_STATEMENT: &str = "UPDATE trainings SET date = ?1 WHERE id = ?2";

        let connection_guard = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_guard.deref();

        let mut statement = connection.prepare(UPDATE_TRAINING_SQL_STATEMENT).unwrap();
        match statement.execute(params![training.date, training_id]) {
            Ok(_) => {
                info!("Update successful");
                for entry in training.entries {
                    self.update_entry(&connection, entry);
                }
            },
            Err(err) => {
                error!("Update failed: {}", err);
                error!("Statement: {}", statement.expanded_sql().unwrap());
            },
        }
    }
}

pub trait ExerciseDetailsOperations {
    fn find_exercise_details(&self, exercise_id: String) -> Vec<ExerciseDetails>;
}

impl ExerciseDetailsOperations for Database {
    fn find_exercise_details(&self, exercise_id: String) -> Vec<ExerciseDetails> {
        const FIND_EXERCISE_DETAILS_SQL_STATEMENT: &str = "
            SELECT training_sets.repetitions, training_sets.weight, trainings.date FROM training_sets
            INNER JOIN training_entries
            ON training_sets.entry_id = training_entries.id
            AND training_entries.exercise_id = ?1
            INNER JOIN trainings
            ON training_entries.training_id = trainings.id
            ORDER BY trainings.date
        ";

        let connection_guard = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_guard.deref();

        let mut statement = connection.prepare(FIND_EXERCISE_DETAILS_SQL_STATEMENT).unwrap();
        let exercise_details_iterator = statement.query_map([exercise_id], |row| {
            return Ok(ExerciseDetails{
                repetitions: row.get(0).unwrap(),
                weight: row.get(1).unwrap(),
                date: row.get(2).unwrap()
            });
        }).unwrap();

        let mut result = Vec::new();
        for exercise_details in exercise_details_iterator {
            result.push(exercise_details.unwrap());
        }
        return result;
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
