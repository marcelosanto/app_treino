use chrono::Local;
use dioxus::prelude::*;

use crate::models::{Exercise, Workoute};

#[component]
pub fn CreateWorkoutModal(workoutes: Signal<Vec<Workoute>>, show_modal: Signal<bool>) -> Element {
    println!("Eu abri");

    let mut exercises = use_signal(|| vec![Exercise::default()]);

    let add_exercise = move |_| {
        exercises.with_mut(|exercises| {
            exercises.push(Exercise::default());
        });
    };

    // Usar signals para armazenar os dados do formulário
    let mut workout_name = use_signal(|| String::new());
    let mut workout_description = use_signal(|| String::new());

    rsx! {
        div {
            id: "createWorkoutModal",
            class: "modal",
            style: "display: block;",
            div { class: "modal-content",
                span {
                    class: "close",
                    onclick: move |_| {
                        show_modal.set(false);
                    },
                    "x"
                }
                h2 { "Criar Novo Plano de Treino" }
                form {
                    id: "createWorkoutForm",
                    onsubmit: move |evt| {
                        evt.prevent_default(); // Evita o comportamento padrão de submissão do formulário

                        // 1. Crie uma nova instância do seu Workoute com os dados dos signals
                        let new_workout = Workoute {
                            id: "100".to_string(),
                            name: workout_name.read().clone(), // .read() para ler o valor, .clone() para passar a propriedade
                            desc: workout_description.read().clone(),
                            // Adapte a data conforme sua lógica
                            date: Local::now(),
                            qtd_exercise: exercises.read().len() as u32,
                            exercises: exercises.read().clone(), // .read() e .clone() para pegar a lista de exercícios
                        };

                        // 2. Use with_mut() para adicionar o novo workout à lista workoutes
                        workoutes

                            // 3. Altere a aba após salvar
                            .with_mut(|workouts| {
                                workouts.push(new_workout);
                            });
                        show_modal.set(false);
                    },
                    div { class: "form-group",
                        label { r#for: "workoutName", "Nome do Treino:" }
                        input {
                            r#type: "text",
                            id: "workoutName",
                            placeholder: "Ex: Treino A - Peito e Tríceps",
                            oninput: move |evt| workout_name.set(evt.value()),
                        }
                    }
                    div { class: "form-group",
                        label { r#for: "workoutDescription", "Descrição (opcional):" }
                        textarea {
                            id: "workoutDescription",
                            rows: "3",
                            placeholder: "Foco do treino, dias da semana...",
                            oninput: move |evt| workout_description.set(evt.value()),
                        }
                    }

                    // ... (restante do código para os exercícios, está correto) ...
                    div { id: "exercisesContainer",
                        h3 { "Exercícios" }
                        for (index , exercise) in exercises().iter().enumerate() {
                            div {
                                key: "{index}",
                                style: "border: 2px solid #e9ecef; border-radius: 10px; padding: 20px; margin: 15px 0; background: #f8f9fa;",
                                div { class: "form-group",
                                    label { "Nome do Exercício:" }
                                    input {
                                        r#type: "text",
                                        class: "exercise-name-input",
                                        required: true,
                                        placeholder: "Ex: Supino Reto com Halteres",
                                        value: "{exercise.name}",
                                        oninput: move |evt| {
                                            exercises
                                                .with_mut(|exercises| {
                                                    exercises[index].name = evt.value();
                                                });
                                        },
                                    }
                                }
                                div { class: "form-group",
                                    label { "Séries Planejadas:" }
                                    input {
                                        r#type: "number",
                                        class: "exercise-sets-input",
                                        min: "1",
                                        value: "{exercise.sets}",
                                        placeholder: "3",
                                        oninput: move |evt| {
                                            if let Ok(value) = evt.value().parse::<u32>() {
                                                exercises
                                                    .with_mut(|exercises| {
                                                        exercises[index].sets = value;
                                                    });
                                            }
                                        },
                                    }
                                }
                                div { class: "form-group",
                                    label { "Repetições Alvo:" }
                                    input {
                                        r#type: "text",
                                        class: "exercise-reps-input",
                                        required: true,
                                        placeholder: "Ex: 8-12",
                                        value: "{exercise.reps}",
                                        oninput: move |evt| {
                                            exercises
                                                .with_mut(|exercises| {
                                                    exercises[index].reps = evt.value();
                                                });
                                        },
                                    }
                                }
                                button {
                                    r#type: "button",
                                    class: "btn btn-danger",
                                    onclick: move |_| {
                                        exercises
                                            .with_mut(|exercises| {
                                                exercises.remove(index);
                                            });
                                    },
                                    "Remover"
                                }
                            }
                        }
                    }
                    button {
                        r#type: "button",
                        class: "btn btn-secondary",
                        onclick: add_exercise,
                        "+ Adicionar Exercício"
                    }
                    button { r#type: "submit", class: "btn btn-primary", "Salvar Plano" }
                }
            }
        }
    }
}
