use dioxus::prelude::*;

use crate::models::RegProgress;

#[component]
pub fn DashBoard(progress: Signal<Vec<RegProgress>>) -> Element {
    println!("DashBoard: {:?}", progress);
    rsx! {
        div { id: "dashboard", class: "tab-content active",
            div { class: "card",
                h2 { "📊 Visão Geral Rápida" }
                div { class: "stats-grid",
                    div { class: "stat-card",
                        div { class: "stat-value", id: "totalWorkouts", "0" }
                        div { class: "stat-label", "Treinos Realizados" }
                    }
                    div { class: "stat-card",
                        div { class: "stat-value", id: "totalVolume", "0" }
                        div { class: "stat-label", "Volume Total (kg)" }
                    }
                    div { class: "stat-card",
                        div { class: "stat-value", id: "totalSets", "0" }
                        div { class: "stat-label", "Sets Completados" }
                    }
                    div { class: "stat-card",
                        div { class: "stat-value", id: "totalExercises", "0" }
                        div { class: "stat-label", "Exercícios Diferentes" }
                    }
                }
            }

            div { class: "card",
                h3 { "🔥 Últimos Treinos Registrados" }
                div { id: "recentWorkouts", class: "workout-list",
                    div { class: "empty-state",
                        p { "Nenhum treino registrado ainda. Comece agora!" }
                    }

                    for prog in progress() {
                        div { class: "workout-item dashboard-item",
                            div { class: "workout-header",
                                div { class: "workout-title", {prog.workout_name} }
                                div { class: "workout-date", {prog.date.to_string()} }
                            }
                            div { class: "exercise-list",
                                for exerc in prog.exercises {
                                    div { style: "margin-bottom: 5px;",
                                        strong { style: "color:white;", {exerc.exercise_name} }
                                        for set in exerc.recorded_sets {
                                            p { "{set.weight}x{set.reps}" }
                                        }
                                                                        //${ex.sets.map(s => `${s.reps}x${s.weight}kg`).join(', ')
                                    }
                                }

                            //     ${log.exercises.map(ex => `
                            //         <div style="margin-bottom: 5px;">
                            //             <strong style="color:white;">${ex.name}:</strong> ${ex.sets.map(s => `${s.reps}x${s.weight}kg`).join(', ')}
                            //         </div>`).join('')}
                            }
                        }
                    }
                }
            }
        }
    }
}
