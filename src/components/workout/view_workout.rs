use dioxus::prelude::*;

// Certifique-se de que Workoute, Exercise, SetData estão importados corretamente
use crate::models::{Exercise, SetData, Workoute};

#[component]
pub fn ViewWorkout(
    selected_workout: Signal<Option<Workoute>>,
    show_modal: Signal<bool>,
) -> Element {
    // 🎯 A correção principal aqui: use `if let` para lidar com o `Option` de forma segura.
    // Se `selected_workout` for `None`, a modal não mostrará o conteúdo do treino.
    if let Some(workout) = selected_workout.read().as_ref() {
        // Criamos uma cópia base do workout para ser usada nas closures.
        // Ela não será movida para as closures, mas clonada individualmente nelas.
        let base_workout_for_closures = workout.clone();

        rsx! {
            div { class: "modal", style: "display: block;", // Garante que o modal esteja visível
                div { class: "modal-content",
                    // Botão para fechar a modal
                    span {
                        class: "close",
                        onclick: move |_| {
                            show_modal.set(false); // Fecha o modal
                            selected_workout.set(None); // Limpa o treino selecionado
                        },
                        "x"
                    }
                    h2 { "{workout.name}" }
                    p { "{workout.desc}" }

                    div { class: "exercise-list",
                        for exercise in &workout.exercises {
                            div {
                                key: "{exercise.id}", // Use o ID do exercício como chave única
                                class: "exercise-item",
                                style: "border: 1px solid #ddd; padding: 10px; margin-bottom: 10px; border-radius: 5px;",
                                div { class: "exercise-name",
                                    h4 { "{exercise.name}" }
                                }
                                p { "Repetições Alvo: {exercise.reps}" }

                                // Detalhes das Séries
                                div { class: "series-details",
                                    h5 { "Séries Planejadas:" }
                                    ul {
                                        for (set_idx , set_data) in exercise.sets_data.iter().enumerate() {
                                            li { key: "{exercise.id}-{set_idx}",
                                                "Série {set_idx + 1}: Peso: {set_data.weight} kg, Repetições: {set_data.reps}"
                                            }
                                        }
                                    }
                                    // Se exercise.sets_data estiver vazio, pode mostrar uma mensagem
                                    if exercise.sets_data.is_empty() {
                                        p { "Nenhuma série definida para este exercício." }
                                    }
                                }
                            }
                        }
                    }

                    // Botões de Ação
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| {
                            // 🎯 CLONE AQUI, DENTRO DA CLOSURE:
                            let workout_to_register = base_workout_for_closures.clone();
                            // Aqui você pode adicionar a lógica para iniciar o registro do treino
                            // Por exemplo, navegar para a aba de progresso e pré-selecionar este treino
                            println!("Registrar treino: {:?}", workout_to_register.name);
                            show_modal.set(false); // Fecha a modal
                            // Você precisaria de um Signal compartilhado para mudar a aba.
                            // Por exemplo: toggle_tabs.set(Tabs::Progress);
                        },
                        "Registrar este Treino"
                    }
                    button {
                        class: "btn btn-danger",
                        // onclick: move |_| {
                        //     // 🎯 CLONE AQUI, DENTRO DA CLOSURE:
                        //     let workout_to_delete = base_workout_for_closures.clone();
                        //     // Lógica para excluir o treino. Isso precisaria do Signal `workoutes` do componente pai.
                        //     println!("Excluir treino: {:?}", workout_to_delete.name);
                        //     show_modal.set(false); // Fecha a modal após a exclusão (ou confirmação)
                        // },
                        style: "margin-left: 10px;",
                        "Excluir Treino"
                    }
                }
            }
        }
    } else {
        // Se `selected_workout` for `None`, a modal não é renderizada ou mostra um placeholder
        rsx! {}
    }
}
