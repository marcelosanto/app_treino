use chrono::{DateTime, Local};
use dioxus::prelude::*;

use crate::models::RegProgress;

#[component]
pub fn DashBoard() -> Element {
    let progress_regs = use_context::<Signal<Vec<RegProgress>>>();

    let soma_reps: i32 = progress_regs() // ou o tipo numérico apropriado para `reps`
        .iter()
        .flat_map(|f| &f.exercises) // Achata a lista de exercícios de cada 'RegProgress'
        .flat_map(|x| &x.recorded_sets) // Achata a lista de séries de cada 'RecordedExerciseProgress'
        .map(|set| set.reps as i32) // Extrai o valor de 'reps' de cada série
        .sum();

    let soma_pesos: i32 = progress_regs() // ou o tipo numérico apropriado para `reps`
        .iter()
        .flat_map(|f| &f.exercises) // Achata a lista de exercícios de cada 'RegProgress'
        .flat_map(|x| &x.recorded_sets) // Achata a lista de séries de cada 'RecordedExerciseProgress'
        .map(|set| set.weight as i32) // Extrai o valor de 'reps' de cada série
        .sum();

    let total_exercicios: usize = progress_regs()
        .iter()
        .map(|reg_progress| reg_progress.exercises.len()) // Para cada treino, obtém o número de exercícios
        .sum();

    rsx! {
        div { id: "dashboard", class: "tab-content active",
            div { class: "card",
                h2 { "📊 Visão Geral Rápida" }
                div { class: "stats-grid",
                    div { class: "stat-card",
                        div { class: "stat-value", "{progress_regs().len()}" }
                        div { class: "stat-label", "Treinos Realizados" }
                    }
                    div { class: "stat-card",
                        div { class: "stat-value", "{soma_pesos}" }
                        div { class: "stat-label", "Volume Total (kg)" }
                    }
                    div { class: "stat-card",
                        div { class: "stat-value", "{soma_reps}" }
                        div { class: "stat-label", "Series Completadas" }
                    }
                    div { class: "stat-card",
                        div { class: "stat-value", "{total_exercicios}" }
                        div { class: "stat-label", "Exercícios Diferentes" }
                    }
                }
            }

            div { class: "card",
                h3 { "🔥 Últimos Treinos Registrados" }
                div { id: "recentWorkouts", class: "workout-list",
                    if progress_regs().is_empty() {
                        div { class: "empty-state",
                            p { "Nenhum treino registrado ainda. Comece agora!" }
                        }
                    }

                    for prog in progress_regs() {
                        div { class: "workout-item dashboard-item",
                            div { class: "workout-header",
                                div { class: "workout-title", {prog.workout_name} }
                                div { class: "workout-date", {format_date(prog.date)} }
                            }
                            div { class: "exercise-list",
                                for exerc in prog.exercises {
                                    div { style: "margin-bottom: 5px;",
                                        strong { style: "color:white;", "{exerc.exercise_name}: " }
                                        for (index , set) in exerc.recorded_sets.iter().enumerate() {
                                            if index == 0 {
                                                "{set.reps}x{set.weight}kg"
                                            } else {
                                                " - {set.reps}x{set.weight}kg "
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn format_date(date: DateTime<Local>) -> String {
    date.format("%d-%m-%Y").to_string()
}
