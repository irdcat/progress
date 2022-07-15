use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct Exercise {
    pub id: String,
    pub name: String,
    pub description: String,
    pub bodyweight: bool,
    pub unilateral: bool,
    pub double_weight: bool
}

#[derive(Deserialize, Debug)]
pub struct ExercisePatch {
    pub name: String,
    pub description: String,
    pub bodyweight: bool,
    pub unilateral: bool,
    pub double_weight: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainingSet {
    pub id: String,
    pub repetitions: i32,
    pub weight: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainingEntry {
    pub id: String,
    pub exercise_id: String,
    pub sets: Vec<TrainingSet>
}

#[derive(Serialize, Debug)]
pub struct Training {
    pub id: String,
    pub date: NaiveDate,
    pub entries: Vec<TrainingEntry>
}

#[derive(Deserialize, Debug)]
pub struct TrainingPatch {
    pub date: NaiveDate,
    pub entries: Vec<TrainingEntry>
}

#[derive(Serialize)]
pub struct ExerciseDetails {
    pub repetitions: i32,
    pub weight: f64,
    pub date: NaiveDate
}
