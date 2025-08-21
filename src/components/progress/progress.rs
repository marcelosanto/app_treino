use chrono::{DateTime, Local};
use dioxus::prelude::*;

use crate::models::Workoute;

struct RegProgress {
    // id: DateTime<Local>, gerar id
    workout_id: u32,
    workout_name: String,
    date: DateTime<Local>,
    exercises: String,
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
                            let workt_find = workout().iter().find(|w| w.id == evt.value()).cloned();
                            selected_progress_workout.set(workt_find);
                        },
                        option { value: "", "Escolha um treino" }

                        for work in workout() {
                            option { value: {work.id}, {work.name} }
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
    rsx! {
        form { id: "progressWorkoutForm",
            div { class: "form-group",
                label { "Data do Treino:" }
                input {
                    r#type: "date",
                    id: "workoutDate",
                    value: "{Local::now()}",
                    required: true,
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
                            for e in 1..=exercise.sets {

                                div { class: "set-input",
                                    span { "Série {e}" }
                                    input {
                                        r#type: "number",
                                        step: "0.5",
                                        min: "0",
                                        class: "set-weight",
                                        placeholder: "Peso (kg)",
                                    }
                                    input {
                                        r#type: "number",
                                        min: "0",
                                        class: "set-reps",
                                        placeholder: "Reps",
                                    }
                                }
                            }
                        
                        }
                        button {
                            r#type: "button",
                            class: "btn btn-secondary",
                            onclick: move |_| { sets += 1 },
                            style: "margin-top:10px;",
                            "+ Adicionar Série"
                        }
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
