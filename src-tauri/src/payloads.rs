use serde::Deserialize;

#[derive(Deserialize)]
pub struct ExercisePayload {
    pub name: Option<String>,
    pub description: Option<String>,
    pub bodyweight: Option<bool>,
}