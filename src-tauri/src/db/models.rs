use serde::Serialize;
use super::schema::*;

#[derive(Serialize, Queryable)]
pub struct Exercise {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub bodyweight: bool,
}

#[derive(Insertable)]
#[table_name="exercises"]
pub struct NewExercise<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub bodyweight: bool,
}

#[derive(AsChangeset)]
#[table_name="exercises"]
pub struct UpdateExercise<'a> {
    pub name: Option<&'a str>,
    pub description: Option<&'a str>,
    pub bodyweight: Option<bool>,
}