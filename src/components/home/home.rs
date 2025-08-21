use chrono::{DateTime, Local};
use dioxus::prelude::*;

use crate::{
    components::{
        dashboard::dashboard::DashBoard,
        progress::progress::Progress,
        stats::stats::Stats,
        workout::{create_workout::CreateWorkoutModal, workout::Workouts},
    },
    models::{Exercise, Tabs, Workoute},
};

#[component]
pub fn Home() -> Element {
    let mut toggle_tabs = use_signal(|| Tabs::DashBoard);
    let mut workoutes = use_signal(|| vec![]);
    let now: DateTime<Local> = Local::now();
    let mut show_modal = use_signal(|| false);

    if workoutes.is_empty() {
        workoutes.push(Workoute {
            id: "1".to_string(),
            name: "Treino A".to_string(),
            desc: "Treino A".to_string(),
            date: now,
            qtd_exercise: 3,
            exercises: vec![
                Exercise {
                    name: "Supino Reto".to_string(),
                    sets: 3,
                    reps: "12".to_string(),
                },
                Exercise {
                    name: "Desenvolvimento".to_string(),
                    sets: 3,
                    reps: "12".to_string(),
                },
            ],
        });

        workoutes.push(Workoute {
            id: "2".to_string(),
            name: "Treino B".to_string(),
            desc: "Costas".to_string(),
            date: now,
            qtd_exercise: 3,
            exercises: vec![
                Exercise {
                    name: "Puxada".to_string(),
                    sets: 3,
                    reps: "12".to_string(),
                },
                Exercise {
                    name: "Remada".to_string(),
                    sets: 3,
                    reps: "12".to_string(),
                },
                Exercise {
                    name: "Remada Curvada".to_string(),
                    sets: 3,
                    reps: "12".to_string(),
                },
            ],
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
                        DashBoard {}
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
                        Progress { workout: workoutes }
                    },
                    Tabs::Stats => rsx! {
                        Stats {}
                    },

                }

                if show_modal() {
                    CreateWorkoutModal { workoutes, show_modal }
                }
            }
        }
    }
}
