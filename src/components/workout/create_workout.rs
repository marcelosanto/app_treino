use chrono::{DateTime, Local};
use dioxus::prelude::*;
use uuid::Uuid;

// Importe suas structs atualizadas
use crate::models::{Exercise, SetData, Workoute};

#[component]
pub fn CreateWorkoutModal(workoutes: Signal<Vec<Workoute>>, show_modal: Signal<bool>) -> Element {
    println!("Eu abri o Modal de Criação de Treino");

    // Signal para a lista de exercícios que estão sendo criados neste modal
    let mut exercises = use_signal(|| vec![Exercise::default()]);

    // Signals para armazenar os dados do formulário principal do treino
    let mut workout_name = use_signal(|| String::new());
    let mut workout_description = use_signal(|| String::new());

    rsx! {
        div {
            id: "createWorkoutModal",
            class: "modal",
            style: "display: block;", // Garante que o modal seja exibido
            div { class: "modal-content",
                span {
                    class: "close",
                    onclick: move |_| {
                        show_modal.set(false); // Fecha o modal
                        // Opcional: Limpar os signals do formulário aqui
                        workout_name.set(String::new());
                        workout_description.set(String::new());
                        exercises.set(vec![Exercise::default()]);
                    },
                    "x"
                }
                h2 { "Criar Novo Plano de Treino" }
                form {
                    id: "createWorkoutForm",
                    onsubmit: move |evt| {
                        evt.prevent_default(); // Evita o comportamento padrão de submissão do formulário (recarregar página)

                        // 1. Cria uma nova instância de Workoute com os dados dos signals
                        let new_workout = Workoute {
                            id: Uuid::new_v4(), // Gera um novo UUID para o treino
                            name: workout_name.read().clone(),
                            desc: workout_description.read().clone(),
                            date: Local::now(), // Data atual de criação
                            qtd_exercise: exercises.read().len() as u32, // Quantidade de exercícios
                            exercises: exercises.read().clone(), // Copia a lista de exercícios (com suas séries)
                        };

                        // 2. Adiciona o novo treino à lista global de treinos
                        workoutes

                            // 3. Limpa o formulário após salvar

                            // 4. Fecha o modal
                            .with_mut(|workouts_vec| {
                                workouts_vec.push(new_workout);
                            });
                        workout_name.set(String::new());
                        workout_description.set(String::new());
                        exercises.set(vec![Exercise::default()]);
                        show_modal.set(false);
                    },
                    // Campos para Nome e Descrição do Treino
                    div { class: "form-group",
                        label { r#for: "workoutName", "Nome do Treino:" }
                        input {
                            r#type: "text",
                            id: "workoutName",
                            placeholder: "Ex: Treino A - Peito e Tríceps",
                            value: "{workout_name}", // Vincula ao signal do nome
                            oninput: move |evt| workout_name.set(evt.value()),
                            required: true, // Torna o campo obrigatório
                        }
                    }
                    div { class: "form-group",
                        label { r#for: "workoutDescription", "Descrição (opcional):" }
                        textarea {
                            id: "workoutDescription",
                            rows: "3",
                            placeholder: "Foco do treino, dias da semana...",
                            value: "{workout_description}", // Vincula ao signal da descrição
                            oninput: move |evt| workout_description.set(evt.value()),
                        }
                    }

                    // Container para a lista de exercícios
                    div { id: "exercisesContainer",
                        h3 { "Exercícios" }
                        // Loop externo: itera sobre cada EXERCÍCIO na lista
                        for (exercise_idx , exercise) in exercises().iter().enumerate() {
                            div {
                                key: "{exercise.id}", // Chave única para cada exercício (UUID)
                                class: "exercise-form-group",
                                style: "border: 2px solid #e9ecef; border-radius: 10px; padding: 20px; margin: 15px 0; background: #f8f9fa;",

                                // Input para o nome do exercício
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
                                                .with_mut(|exercises_vec| {
                                                    exercises_vec[exercise_idx].name = evt.value();
                                                });
                                        },
                                    }
                                }
                                // Input para as Repetições Alvo (geral para o exercício)
                                div { class: "form-group",
                                    label { "Repetições Alvo (geral):" }
                                    input {
                                        r#type: "text",
                                        class: "exercise-reps-input",
                                        required: true,
                                        placeholder: "Ex: 8-12",
                                        value: "{exercise.reps}",
                                        oninput: move |evt| {
                                            exercises
                                                .with_mut(|exercises_vec| {
                                                    exercises_vec[exercise_idx].reps = evt.value();
                                                });
                                        },
                                    }
                                }

                                // Loop interno: itera sobre cada SÉRIE (SetData) DESTE exercício
                                for (set_idx , set_data) in exercise.sets_data.iter().enumerate() {
                                    div {
                                        key: "{exercise.id}-{set_idx}", // Chave única para cada série dentro do exercício
                                        class: "set-input",
                                        style: "border: 1px dashed #ced4da; padding: 10px; margin-top: 10px; border-radius: 5px; display: flex; align-items: center; gap: 10px;",
                                        span { "Série {set_idx + 1}" }

                                        // Input para Peso da Série
                                        input {
                                            r#type: "number",
                                            step: "0.5",
                                            min: "0",
                                            class: "set-weight",
                                            placeholder: "Peso (kg)",
                                            required: true,
                                            value: "{set_data.weight}", // Vincula ao peso da série
                                            oninput: move |evt| {
                                                if let Ok(value) = evt.value().parse::<f32>() {
                                                    exercises
                                                        .with_mut(|exercises_vec| {
                                                            exercises_vec[exercise_idx].sets_data[set_idx].weight = value;
                                                        });
                                                }
                                            },
                                        }
                                        // Input para Repetições da Série
                                        input {
                                            r#type: "number",
                                            min: "0",
                                            class: "set-reps",
                                            placeholder: "Reps",
                                            required: true,
                                            value: "{set_data.reps}", // Vincula às repetições da série
                                            oninput: move |evt| {
                                                if let Ok(value) = evt.value().parse::<u32>() {
                                                    exercises
                                                        .with_mut(|exercises_vec| {
                                                            exercises_vec[exercise_idx].sets_data[set_idx].reps = value;
                                                        });
                                                }
                                            },
                                        }
                                        // Botão para remover esta série específica
                                        button {
                                            r#type: "button",
                                            class: "btn btn-danger btn-sm",
                                            onclick: move |_| {
                                                exercises
                                                    .with_mut(|exercises_vec| {
                                                        exercises_vec[exercise_idx].sets_data.remove(set_idx);
                                                    });
                                            },
                                            "X"
                                        }
                                    }
                                }
                                // Botão para adicionar uma nova SÉRIE a ESTE exercício
                                button {
                                    r#type: "button",
                                    class: "btn btn-secondary btn-sm",
                                    onclick: move |_| {
                                        exercises
                                            .with_mut(|exercises_vec| {
                                                exercises_vec[exercise_idx].sets_data.push(SetData::default());
                                            });
                                    },
                                    style: "margin-top:10px;",
                                    "+ Adicionar Série"
                                }

                                // Botão para remover ESTE EXERCÍCIO completo
                                button {
                                    r#type: "button",
                                    class: "btn btn-danger",
                                    onclick: move |_| {
                                        exercises
                                            .with_mut(|exercises_vec| {
                                                exercises_vec.remove(exercise_idx);
                                            });
                                    },
                                    "Remover Exercício"
                                }
                            }
                        }
                    }
                    // Botão para adicionar um NOVO EXERCÍCIO completo ao plano
                    button {
                        r#type: "button",
                        class: "btn btn-secondary",
                        onclick: move |_| {
                            exercises
                                .with_mut(|exercises_vec| {
                                    exercises_vec.push(Exercise::default());
                                });
                        },
                        "+ Adicionar Exercício"
                    }
                    button { r#type: "submit", class: "btn btn-primary", "Salvar Plano" }
                }
            }
        }
    }
}
