pub mod models;
pub mod schema;

use std::{sync::{Arc, Mutex}, ops::Deref};
use uuid::Uuid;
use models::*;
use schema::test;
use schema::test::dsl::*;
use diesel::prelude::*;

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

    pub fn get_test(&self) -> Vec<Test> {
        let connection_mutex = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_mutex.deref();
        return test.load::<Test>(connection).expect("Error loading data");
    }
    
    pub fn create_test(&self, test_name: &str) -> String {
        let connection_mutex = self.connection_arc_mutex.lock().unwrap();
        let connection = connection_mutex.deref();
        let uuid = Uuid::new_v4().as_hyphenated().to_string();

        let new_test = NewTest { id: &uuid, name: test_name };
    
        diesel::insert_into(test::table)
            .values(&new_test)
            .execute(connection)
            .expect("Error creating new data");

        return uuid;
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
