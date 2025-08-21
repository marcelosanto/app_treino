use chrono::{DateTime, Local};
use uuid::Uuid;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Exercise {
    pub id: Uuid,
    pub name: String,
    pub sets: u32,
    pub reps: String,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Workoute {
    pub id: Uuid,
    pub name: String,
    pub desc: String,
    pub date: DateTime<Local>,
    pub qtd_exercise: u32,
    pub exercises: Vec<Exercise>,
}
