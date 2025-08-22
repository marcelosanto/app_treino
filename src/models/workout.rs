use chrono::{DateTime, Local};
use uuid::Uuid;

#[derive(Clone, PartialEq, Debug)]
pub struct Workoute {
    pub id: Uuid,
    pub name: String,
    pub desc: String,
    pub date: DateTime<Local>,
    pub qtd_exercise: u32,
    pub exercises: Vec<Exercise>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Exercise {
    pub id: Uuid,
    pub name: String,
    pub reps: String,            // Repetições alvo (ex: "8-12")
    pub sets_data: Vec<SetData>, // Lista de séries com seus dados de peso/reps
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct SetData {
    pub weight: f32,
    pub reps: u32,
}

impl Default for Exercise {
    fn default() -> Self {
        Exercise {
            id: Uuid::new_v4(),
            name: String::new(),
            reps: String::new(),
            sets_data: vec![SetData::default()], // Começa com uma série vazia por padrão
        }
    }
}

impl Default for Workoute {
    fn default() -> Self {
        Workoute {
            id: Uuid::new_v4(),
            name: String::new(),
            desc: String::new(),
            date: Local::now(),
            qtd_exercise: 0,
            exercises: vec![],
        }
    }
}
