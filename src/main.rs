use chrono::{DateTime, Local};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Tabs {
    DashBoard,
    Workouts,
    Progress,
    Stats,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Exercise {
    name: String,
    sets: u32,
    reps: String,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Workoute {
    name: String,
    desc: String,
    date: DateTime<Local>,
    qtd_exercise: u32,
    exercises: Vec<Exercise>,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Home {}
    }
}

/// Home page
#[component]
fn Home() -> Element {
    let mut toggle_tabs = use_signal(|| Tabs::DashBoard);
    let mut workoutes = use_signal(|| vec![]);
    let now: DateTime<Local> = Local::now();
    let mut show_modal = use_signal(|| false);

    if workoutes.is_empty() {
        workoutes.push(Workoute {
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
                    },
                    Tabs::Workouts => rsx! {
                        Workouts { workoutes }
                    },
                    Tabs::Progress => rsx! {
                        Progress {}
                    },
                    Tabs::Stats => rsx! {
                        Stats {}
                    },

                }

                if show_modal() {
                    CreateWorkoutModal { workoutes, show_modal }
                }

                button {
                    class: "floating-add-btn",
                    onclick: move |_| show_modal.set(true),
                    title: "Criar novo plano de treino",
                    "+"
                }
            }
        }
    }
}

#[component]
pub fn DashBoard() -> Element {
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
                }
            }
        }
    }
}

#[component]
pub fn Workouts(workoutes: Signal<Vec<Workoute>>) -> Element {
    println!("Workouts -> {:?}", workoutes());

    let show_modal = use_signal(|| false);
    let selected_workout = use_signal(|| None);

    rsx! {
        div {
            div { class: "card",
                h2 { "💪 Meus Planos de Treino" }
                p { style: "margin-bottom: 20px; color: #555;",
                    "Crie seus planos de treino aqui. Depois, vá para a aba "Registrar
                    " para lançar seus resultados."
                }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| { println!("Criar novo treino") },
                    "+ Criar Novo Plano de Treino"
                }
                div { id: "workoutsList", class: "workout-list",
                    if workoutes.is_empty() {
                        div { class: "empty-state",
                            p { "Você ainda não tem planos de treino. Crie o primeiro!" }
                        }
                    } else {

                        div {
                            for work in workoutes() {

                                ListWorkout {
                                    work,
                                    show_modal,
                                    selected_workout,
                                }
                            }
                        }

                        if show_modal() {
                            ViewWorkout { work: selected_workout, show_modal }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ListWorkout(
    work: Workoute,
    show_modal: Signal<bool>,
    selected_workout: Signal<Option<Workoute>>,
) -> Element {
    println!("ListWorkout {:?}", work.name);
    let formatted_date = work.date.format("%d/%m").to_string();
    let cloned_workout = work.clone();
    rsx! {
        div {
            class: "workout-item",
            onclick: move |_| {
                selected_workout.set(Some(cloned_workout.clone()));
                show_modal.set(true)
            },
            div { class: "workout-header",
                div { class: "workout-title", "{work.name}" }
                div { class: "workout-date", "Criado em {formatted_date}" }
            }
            p {
                if work.desc != "" {
                    "{work.desc}"
                } else {
                    "Sem descrição"
                }
            }
            div { style: "margin-top: 10px; font-size: 0.9rem; opacity: 0.8;",
                "{work.qtd_exercise} exercícios"
            }
        }
    }
}

#[component]
pub fn ViewWorkout(work: Signal<Option<Workoute>>, show_modal: Signal<bool>) -> Element {
    let work = work.unwrap();
    rsx! {
        div { class: "modal", style: "display: block;",
            div { class: "modal-content",
                span { class: "close", onclick: move |_| show_modal.set(false), "x" }
                h2 { {work.name} }
                p { {work.desc} }
                div { class: "exercise-list",
                    for exercise in work.exercises {
                        div { class: "exercise-item",
                            div { class: "exercise-name", "{exercise.name}" }
                            p { "Meta: {exercise.sets} séries de {exercise.reps} reps" }
                        }
                    }



                    button { class: "btn btn-primary",
                        //onclick:"startWorkout(${workout.id})",
                        "Registrar este Treino"
                    }
                    button {
                        class: "btn btn-danger",
                        //onclick:"deleteWorkout(${workout.id})",
                        style: "margin-left: 10px;",
                        "Excluir Treino"
                    }
                }
            }
        }
    }
}

#[component]
pub fn Progress() -> Element {
    rsx! {
        div { id: "progress", class: "tab-content active",
            div { class: "card",
                h2 { "📈 Registrar Progresso" }
                div { class: "form-group",
                    label { r#for: "progressWorkout", "Selecione o Plano de Treino de hoje:" }
                    select { id: "progressWorkout",
                        // onchange: {"loadWorkoutForProgress()"},
                        option { value: "", "Escolha um treino" }
                    }
                }
                div { id: "progressForm" }
            }
        }
    }
}

#[component]
pub fn Stats() -> Element {
    rsx! {
        div { id: "stats", class: "tab-content active",
            div { class: "card",
                h2 { "🏆 Recordes Pessoais (PRs)" }
                div { id: "detailedStats",
                    div { class: "empty-state",
                        p { "Complete alguns treinos para ver suas estatísticas detalhadas!" }
                    }
                }
            }
        }
    }
}

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
