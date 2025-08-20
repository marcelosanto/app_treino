use chrono::{DateTime, Local};

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Exercise {
    pub name: String,
    pub sets: u32,
    pub reps: String,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Workoute {
    pub name: String,
    pub desc: String,
    pub date: DateTime<Local>,
    pub qtd_exercise: u32,
    pub exercises: Vec<Exercise>,
}
