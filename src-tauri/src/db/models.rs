use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct Exercise {
    pub id: String,
    pub name: String,
    pub description: String,
    pub bodyweight: bool
}

#[derive(Deserialize, Debug)]
pub struct ExercisePatch {
    pub name: String,
    pub description: String,
    pub bodyweight: bool
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
    pub date: String,
    pub entries: Vec<TrainingEntry>
}

#[derive(Deserialize, Debug)]
pub struct TrainingPatch {
    pub date: String,
    pub entries: Vec<TrainingEntry>
}