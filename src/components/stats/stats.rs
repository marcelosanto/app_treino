use std::collections::HashMap;

use dioxus::prelude::*;

use crate::models::RegProgress;

#[derive(Debug, Clone)]
pub struct ExerciseRecord {
    pub exercise_name: String,
    pub max_weight: f32,
    pub max_volume: f32,
}

#[component]
pub fn Stats() -> Element {
    let records = encontrar_recordes();

    rsx! {
        div { id: "stats", class: "tab-content active",
            div { class: "card",
                h2 { "🏆 Recordes Pessoais (PRs)" }
                div { id: "detailedStats",
                    if records.is_empty() {
                        div { class: "empty-state",
                            p { "Complete alguns treinos para ver suas estatísticas detalhadas!" }
                        }
                    }
                    div { class: "stats-grid",
                        for recs in records {
                            div { class: "card",
                                h4 { {recs.exercise_name} }
                                p {
                                    strong { "Carga Máxima:" }
                                    "{recs.max_weight} kg"
                                }
                                p {
                                    strong { "Melhor Série (Volume):" }
                                    "{recs.max_volume} kg"
                                }
                            }
                        }

                    }
                }
                p { style: "text-align:center; margin-top:20px; color:#555;",
                    i { "Em breve: Gráficos de progressão!" }
                }

            }
        }
    }
}

fn encontrar_recordes() -> Vec<ExerciseRecord> {
    let mut records: HashMap<String, ExerciseRecord> = HashMap::new();
    let progress_regs = use_context::<Signal<Vec<RegProgress>>>();

    for reg_progress in progress_regs.iter() {
        for exercise in &reg_progress.exercises {
            for set in &exercise.recorded_sets {
                // volume = peso * repetições
                let current_volume = set.weight * set.reps as f32;
                let current_weight = set.weight;

                records
                    .entry(exercise.exercise_name.clone())
                    .and_modify(|existing_record| {
                        if current_weight > existing_record.max_weight {
                            existing_record.max_weight = current_weight;
                        }
                        if current_volume > existing_record.max_volume {
                            existing_record.max_volume = current_volume;
                        }
                    })
                    .or_insert_with(|| ExerciseRecord {
                        exercise_name: exercise.exercise_name.clone(),
                        max_weight: current_weight,
                        max_volume: current_volume,
                    });
            }
        }
    }

    records.into_values().collect()
}
