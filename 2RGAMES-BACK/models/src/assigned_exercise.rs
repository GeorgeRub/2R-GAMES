use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

static ASSIGNED_EXERCISE_TABLE_NAME: &str = "AssignedExercise";

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AssignedExercise {
    pub assigned_exercise_id: String,
    pub user_email: String,
    pub doc_email: String,
    pub data_completed: DateTime<Utc>,
    pub exercise_id: String,
    pub repetitions: i32,
    pub approach: i8,
    pub done: bool,
    pub data_finished: Option<DateTime<Utc>>,
}

impl AssignedExercise{
    
}