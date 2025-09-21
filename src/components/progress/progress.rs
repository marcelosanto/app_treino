use chrono::Local;
use dioxus::prelude::*;
use uuid::Uuid;

use crate::models::{RecordedExerciseProgress, RegProgress, SetData, Tabs, Workoute}; // Certifique-se de que Exercise e SetData estão importados

#[component]
pub fn Progress() -> Element {
    let mut selected_progress_workout = use_signal(|| None::<Workoute>);
    let workoutes = use_context::<Signal<Vec<Workoute>>>();
    let selected_workout_for_register = use_context::<Signal<Workoute>>();
    let mut workout_name = use_signal(|| "Escolha um treino".to_string());

    use_effect(move || {
        let workout_from_context = selected_workout_for_register.read();

        if !workout_from_context.name.is_empty() {
            workout_name.set(workout_from_context.name.clone());
            selected_progress_workout.set(Some(workout_from_context.clone()));
        }
    });

    rsx! {
        div { class: "tab-content active",
            div { class: "card",
                h2 { "📈 Registrar Progresso" }
                div { class: "form-group",
                    label { r#for: "progressWorkout", "Selecione o Plano de Treino de hoje:" }
                    select {
                        oninput: move |evt| {
                            println!("Selected value: {}", evt.value());

                            let parsed_uuid_option = Uuid::parse_str(&evt.value()).ok();

                            if let Some(uuid) = parsed_uuid_option {
                                let workt_find = workoutes.read().iter().find(|w| w.id == uuid).cloned();
                                selected_progress_workout.set(workt_find);
                            } else {
                                selected_progress_workout.set(None); // Limpa a seleção se for "Escolha um treino"
                            }
                        },
                        option { value: "", "{workout_name}" }

                        for work in workoutes() {
                            option { value: "{work.id}", "{work.name}" } // Use o ID como valor
                        }
                    }
                }
                div {
                    {
                        match selected_progress_workout() {
                            Some(_) => rsx! {
                                FormProgress { workout_to_record: selected_progress_workout }
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

// O FormProgress agora recebe um Signal<Option<Workoute>> para poder modificá-lo
#[component]
pub fn FormProgress(workout_to_record: Signal<Option<Workoute>>) -> Element {
    let today_date = Local::now().format("%Y-%m-%d").to_string();
    let mut progress_regs = use_context::<Signal<Vec<RegProgress>>>();
    let mut toggle_tabs = use_context::<Signal<Tabs>>();

    rsx! {
        // Precisa desempacotar o Option<Workoute> antes de renderizar o formulário
        {
            if let Some(workout) = workout_to_record.read().as_ref().cloned() {
                rsx! {
                    form {
                        id: "progressWorkoutForm",
                        onsubmit: move |evt| {
                            evt.prevent_default();
                            println!("Dentro do form");
                            println!("Progresso registrado para: {:?}", workout.name);
                            println!("Dados do progresso: {:?}", workout_to_record());



                            let reg_id = Uuid::new_v4();
                            let workout_id = workout.id;
                            let workout_name = workout.name.clone();
                            let reg_date = Local::now(); // Ou a data do input do formulário
                            let exercises_progress: Vec<RecordedExerciseProgress> = workout

                                .exercises
                                .iter()
                                .map(|ex| {
                                    RecordedExerciseProgress {
                                        exercise_id: ex.id,
                                        exercise_name: ex.name.clone(),
                                        recorded_sets: ex.sets_data.clone(),
                                    }
                                })
                                .collect();
                            let final_reg_progress = RegProgress {
                                id: reg_id,
                                workout_id,
                                workout_name,
                                date: reg_date,
                                exercises: exercises_progress,
                            };
                            println!("Objeto final de RegProgress: {:?}", final_reg_progress.id);

                            progress_regs.push(final_reg_progress);
                            toggle_tabs.set(Tabs::DashBoard);
                        },
                        div { class: "form-group",
                            label { "Data do Treino:" }
                            input {
                                r#type: "date",
                                id: "workoutDate",
                                value: "{today_date}", // Define a data atual como valor padrão
                                required: true,
                                oninput: move |evt| println!("Data {}", evt.value()),
                            }
                        }
                        div { id: "exerciseProgress",
                            for (exercise_idx , exercise) in workout.exercises.iter().enumerate() {
                                div {
                                    key: "progress-exercise-{exercise.id}", // Chave usando o ID do exercício
                                    class: "exercise-progress",
                                    style: "border: 2px solid #e9ecef; border-radius:10px; padding:20px; margin: 15px 0;",
                                    h4 { "{exercise.name}" }
                                    p { style: "opacity:0.8; margin-bottom:15px;",
                                        "Meta: {exercise.sets_data.len()} séries de {exercise.reps} reps"
                                    }
                                    div { class: "sets-container", id: "sets-{exercise.id}", // ID único para o container de sets
                                        for (set_idx , set_data) in exercise.sets_data.iter().enumerate() {
                                            div {
                                                key: "progress-set-{exercise.id}-{set_idx}", // Chave única para cada série
                                                class: "set-input",
                                                style: "border: 1px dashed #ced4da; padding: 10px; margin-top: 10px; border-radius: 5px; display: flex; align-items: center; gap: 10px;",
                                                span { "Série {set_idx + 1}" }
                                                input {
                                                    r#type: "number",
                                                    step: "0.5",
                                                    min: "0",
                                                    class: "set-weight",
                                                    placeholder: "Peso (kg)",
                                                    required: true,
                                                    value: "{set_data.weight}", // Vincula diretamente ao valor
                                                    oninput: move |evt| {
                                                        if let Ok(value) = evt.value().parse::<f32>() {
                                                            workout_to_record
                                                                .with_mut(|opt_w| {
                                                                    if let Some(w_mut) = opt_w {
                                                                        w_mut.exercises[exercise_idx].sets_data[set_idx].weight = value;
                                                                    }
                                                                });
                                                        }
                                                    },
                                                }
                                                input {
                                                    r#type: "number",
                                                    min: "0",
                                                    class: "set-reps",
                                                    placeholder: "Reps",
                                                    required: true,
                                                    value: "{set_data.reps}", // Vincula diretamente ao valor
                                                    oninput: move |evt| {
                                                        if let Ok(value) = evt.value().parse::<u32>() {
                                                            workout_to_record
                                                                .with_mut(|opt_w| {
                                                                    if let Some(w_mut) = opt_w {
                                                                        w_mut.exercises[exercise_idx].sets_data[set_idx].reps = value;
                                                                    }
                                                                });
                                                        }
                                                    },
                                                }
                                                button {
                                                    r#type: "button",
                                                    class: "btn btn-danger btn-sm",
                                                    onclick: move |_| {
                                                        workout_to_record
                                                            .with_mut(|opt_w| {
                                                                if let Some(w_mut) = opt_w {
                                                                    w_mut.exercises[exercise_idx].sets_data.remove(set_idx);
                                                                }
                                                            });
                                                    },
                                                    "X"
                                                }
                                            }
                                        }
                                        button {
                                            r#type: "button",
                                            class: "btn btn-secondary",
                                            onclick: move |_| {
                                                workout_to_record
                                                    .with_mut(|opt_w| {
                                                        if let Some(w_mut) = opt_w {
                                                            w_mut.exercises[exercise_idx].sets_data.push(SetData::default());
                                                        }
                                                    });
                                            },
                                            style: "margin-top:10px;",
                                            "+ Adicionar Série Extra"
                                        }
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
            } else {
                rsx! {
                    p { "Nenhum treino selecionado para registrar o progresso." }
                }
            }
        }
    }
}
