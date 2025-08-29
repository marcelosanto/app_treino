use chrono::{DateTime, Local};
use dioxus::prelude::*;
use uuid::Uuid;

use crate::{
    components::{
        dashboard::dashboard::DashBoard,
        progress::progress::Progress,
        stats::stats::Stats,
        workout::{create_workout::CreateWorkoutModal, workout::Workouts},
    },
    models::{workout::SetData, Exercise, RecordedExerciseProgress, RegProgress, Tabs, Workoute},
};

#[component]
pub fn Home() -> Element {
    let mut toggle_tabs = use_signal(|| Tabs::DashBoard);
    let mut workoutes = use_signal(|| vec![]);
    let now: DateTime<Local> = Local::now();
    let mut show_modal = use_signal(|| false);

    let mut progress_regs = use_signal(|| vec![]);

    if progress_regs.is_empty() {
        use_effect(move || {
            progress_regs.push(RegProgress {
                id: Uuid::new_v4(),
                workout_id: Uuid::new_v4(),
                workout_name: "Treino Peito".to_string(),
                date: now,
                exercises: vec![RecordedExerciseProgress {
                    exercise_id: Uuid::new_v4(),
                    exercise_name: "Supino Reto".to_string(),
                    recorded_sets: vec![
                        SetData {
                            weight: 10.0,
                            reps: 12,
                        },
                        SetData {
                            weight: 15.0,
                            reps: 12,
                        },
                        SetData {
                            weight: 20.0,
                            reps: 8,
                        },
                    ],
                }],
            });
        });
    }

    if workoutes.is_empty() {
        use_effect(move || {
            workoutes.push(Workoute {
                id: Uuid::new_v4(),
                name: "Treino A".to_string(),
                desc: "Peito".to_string(),
                date: now,
                qtd_exercise: 3,
                exercises: vec![
                    Exercise {
                        id: Uuid::new_v4(),
                        name: "Supino Reto".to_string(),
                        sets_data: vec![SetData::default()],
                        reps: "12".to_string(),
                    },
                    Exercise {
                        id: Uuid::new_v4(),
                        name: "Desenvolvimento".to_string(),
                        sets_data: vec![SetData::default()],
                        reps: "12".to_string(),
                    },
                ],
            });

            workoutes.push(Workoute {
                id: Uuid::new_v4(),
                name: "Treino B".to_string(),
                desc: "Costas".to_string(),
                date: now,
                qtd_exercise: 3,
                exercises: vec![
                    Exercise {
                        id: Uuid::new_v4(),
                        name: "Puxada".to_string(),
                        sets_data: vec![SetData::default()],
                        reps: "12".to_string(),
                    },
                    Exercise {
                        id: Uuid::new_v4(),
                        name: "Remada".to_string(),
                        sets_data: vec![SetData::default()],
                        reps: "12".to_string(),
                    },
                    Exercise {
                        id: Uuid::new_v4(),
                        name: "Remada Curvada".to_string(),
                        sets_data: vec![SetData::default()],
                        reps: "12".to_string(),
                    },
                ],
            });
        });
    }

    rsx! {
        head {
            meta { charset: "UTF-8" }
            meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1.0",
            }

            title { "GymTracker Pro" }
        }
        body {
            div { class: "container",
                div { class: "header",
                    h1 { "🏋️ GymTracker Pro" }
                    p { "Seu companheiro para treinos mais eficientes" }
                }

                div { class: "nav-tabs",
                    button {
                        class: if toggle_tabs() == Tabs::DashBoard { "nav-tab active" } else { "nav-tab" },
                        onclick: move |_| { toggle_tabs.set(Tabs::DashBoard) },
                        "Dashboard"
                    }
                    button {
                        class: if toggle_tabs() == Tabs::Workouts { "nav-tab active" } else { "nav-tab" },
                        onclick: move |_| { toggle_tabs.set(Tabs::Workouts) },
                        "Meus Treinos"
                    }
                    button {
                        class: if toggle_tabs() == Tabs::Progress { "nav-tab active" } else { "nav-tab" },
                        onclick: move |_| { toggle_tabs.set(Tabs::Progress) },
                        "Registrar"
                    }
                    button {
                        class: if toggle_tabs() == Tabs::Stats { "nav-tab active" } else { "nav-tab" },
                        onclick: move |_| {
                            toggle_tabs.set(Tabs::Stats);
                            println!("Stats")
                        },
                        "Estatísticas"
                    }
                }


                match toggle_tabs() {
                    Tabs::DashBoard => rsx! {
                        DashBoard { progress: progress_regs }
                        button {
                            class: "floating-add-btn",
                            onclick: move |_| show_modal.set(true),
                            title: "Criar novo plano de treino",
                            "+"
                        }
                    },
                    Tabs::Workouts => rsx! {
                        Workouts { workoutes }
                    },
                    Tabs::Progress => rsx! {
                        Progress { all_workouts: workoutes, reg_progress: progress_regs }
                    },
                    Tabs::Stats => rsx! {
                        Stats { progress: progress_regs }
                    },

                }

                if show_modal() {
                    CreateWorkoutModal { workoutes, show_modal }
                }
            }
        }
    }
}
