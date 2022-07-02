use serde::Serialize;

#[derive(Serialize)]
pub struct Exercise {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub bodyweight: bool,
}

pub struct NewExercise {
    pub name: String,
    pub description: Option<String>,
    pub bodyweight: bool,
}

pub struct UpdateExercise {
    pub name: Option<String>,
    pub description: Option<String>,
    pub bodyweight: Option<bool>,
}