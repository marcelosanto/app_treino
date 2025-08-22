use chrono::{DateTime, Local};
use dioxus::prelude::*;
use uuid::Uuid;

use crate::models::Workoute;

struct RegProgress {
    id: Uuid,
    workout_id: Uuid,
    workout_name: String,
    date: DateTime<Local>,
    exercises: Vec<String>,
}

#[component]
pub fn Progress(workout: Signal<Vec<Workoute>>) -> Element {
    let mut selected_progress_workout = use_signal(|| None::<Workoute>);

    rsx! {
        div { id: "progress", class: "tab-content active",
            div { class: "card",
                h2 { "📈 Registrar Progresso" }
                div { class: "form-group",
                    label { r#for: "progressWorkout", "Selecione o Plano de Treino de hoje:" }
                    select {
                        id: "progressWorkout",
                        oninput: move |evt| {
                            println!("{}", evt.value());

                            let parsed_uuid_option = Uuid::parse_str(&evt.value()).ok();

                            if let Some(uuid) = parsed_uuid_option {

                                let workt_find = workout().iter().find(|w| w.id == uuid).cloned();
                                selected_progress_workout.set(workt_find);
                            }
                        },
                        option { value: "", "Escolha um treino" }

                        for work in workout() {
                            option { value: work.id.to_string(), {work.name} }
                        }
                    }
                }
                div { id: "progressForm",
                    {
                        match selected_progress_workout() {
                            Some(w) => rsx! {
                                FormProgress { workout: w }
                            },
                            None => rsx! {
                                p { "Nenhum treino escolhido" }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn FormProgress(workout: Workoute) -> Element {
    let mut meu_vec: Signal<Vec<(String, String)>> = use_signal(|| Vec::new());

    rsx! {
        form {
            id: "progressWorkoutForm",
            onsubmit: move |evt| {
                evt.prevent_default();
                println!("Dentro do form");
                println!("{:?}", meu_vec);
            },
            div { class: "form-group",
                label { "Data do Treino:" }
                input {
                    r#type: "date",
                    id: "workoutDate",
                    value: "{Local::now()}",
                    required: true,
                    oninput: move |evt| println!("Data {}", evt.value()),
                } //colocar o data do dia
            }
            div { id: "exerciseProgress",
                //pecorrer os exercicios
                for exercise in workout.exercises {
                    div {
                        class: "exercise-progress",
                        style: "border: 2px solid #e9ecef; border-radius:10px; padding:20px; margin: 15px 0;",
                        h4 { "{exercise.name}" }
                        p { style: "opacity:0.8; margin-bottom:15px;",
                            "Meta: {exercise.sets} séries de {exercise.reps} reps"
                        }
                        div { class: "sets-container", id: "sets-1",
                            //pecorrer as series dos exercicios
                            for e in 0..exercise.sets {
                                div { class: "set-input",
                                    span { "Série {e}" }
                                    input {
                                        r#type: "number",
                                        step: "0.5",
                                        min: "0",
                                        class: "set-weight",
                                        placeholder: "Peso (kg)",
                                        required: true,
                                        oninput: move |evt| {
                                            meu_vec.push((exercise.id.to_string(), evt.value()));
                                            println!("Serie{e} -> {} - {} kg", evt.value(), exercise.id)
                                        },
                                    }
                                    input {
                                        r#type: "number",
                                        min: "0",
                                        class: "set-reps",
                                        placeholder: "Reps",
                                        required: true,
                                        oninput: move |evt| println!("Serie{e} -> {} reps", evt.value()),
                                    }
                                }
                            }

                            {
                                println!("{:?}", meu_vec);
                            }
                        }
                                        // button {
                    //     r#type: "button",
                    //     class: "btn btn-secondary",
                    //     onclick: move |_| { exercise.sets += 1 },
                    //     style: "margin-top:10px;",
                    //     "+ Adicionar Série"
                    // }
                    }
                }
            }
            button {
                r#type: "submit",
                class: "btn btn-primary",
                style: "width:100%; padding: 15px; font-size: 16px;",
                "Salvar Treino Realizado"
            }
        }
    }
}
