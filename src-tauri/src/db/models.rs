use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Exercise {
    pub id: String,
    pub name: String,
    pub description: String,
    pub bodyweight: bool,
}

#[derive(Deserialize, Debug)]
pub struct ExercisePatch {
    pub name: String,
    pub description: String,
    pub bodyweight: bool,
}