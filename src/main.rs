use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Tabs {
    DashBoard,
    Workouts,
    Progress,
    Stats,
    Modal,
}

#[derive(Default, Clone, PartialEq)]
struct Exercise {
    name: String,
    sets: u32,
    reps: String,
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
    //let mut counter = use_signal(|| 0);

    let mut toggle_tabs = use_signal(|| Tabs::DashBoard);

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
                        Workouts {}
                    },
                    Tabs::Progress => rsx! {
                        Progress {}
                    },
                    Tabs::Stats => rsx! {
                        Stats {}
                    },
                    Tabs::Modal => rsx! {
                        CreateWorkoutModal { toggle_tabs }
                    },
                }

                button {
                    class: "floating-add-btn",
                    onclick: move |_| toggle_tabs.set(Tabs::Modal),
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
pub fn Workouts() -> Element {
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
                    div { class: "empty-state",
                        p { "Você ainda não tem planos de treino. Crie o primeiro!" }
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
pub fn CreateWorkoutModal(toggle_tabs: Signal<Tabs>) -> Element {
    let mut exercises = use_signal(|| vec![Exercise::default()]);

    let add_exercise = move |_| {
        exercises.with_mut(|exercises| {
            exercises.push(Exercise::default());
        });
    };

    rsx! {
        div { id: "createWorkoutModal", class: "modal",
            div { class: "modal-content",
                span {
                    class: "close",
                    onclick: move |_| {
                        toggle_tabs.set(Tabs::Workouts);
                    },
                    "x"
                }
                h2 { "Criar Novo Plano de Treino" }
                form { id: "createWorkoutForm",
                    div { class: "form-group",
                        label { r#for: "workoutName", "Nome do Treino:" }
                        input {
                            r#type: "text",
                            id: "workoutName",
                            placeholder: "Ex: Treino A - Peito e Tríceps",
                        }
                    }
                    div { class: "form-group",
                        label { r#for: "workoutDescription", "Descrição (opcional):" }
                        textarea {
                            id: "workoutDescription",
                            rows: "3",
                            placeholder: "Foco do treino, dias da semana...",
                        }
                    }

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
                    button {
                        r#type: "submit",
                        onclick: move |_| {
                            toggle_tabs.set(Tabs::Workouts);
                        },
                        class: "btn btn-primary",
                        "Salvar Plano"
                    }
                }
            }
        }
    }
}
