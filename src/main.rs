use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/workouts")]
    Workouts {}
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
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
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "links",
                a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                    "💫 VSCode Extension"
                }
                a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    //let mut counter = use_signal(|| 0);

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
                    button { class: "nav-tab active",
                        //onclick: move |_| { "showTab(event, 'dashboard')" },
                        "Dashboard"
                    }
                    button { class: "nav-tab",
                        //onclick: "showTab(event, 'workouts')",
                        "Meus Treinos"
                    }
                    button { class: "nav-tab",
                        //onclick: "showTab(event, 'progress')",
                        "Registrar"
                    }
                    button { class: "nav-tab",
                        //onclick: "showTab(event, 'stats')",
                        "Estatísticas"
                    }
                }

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

                div { id: "workouts", class: "tab-content",
                    div { class: "card",
                        h2 { "💪 Meus Planos de Treino" }
                        p { style: "margin-bottom: 20px; color: #555;",
                            "Crie seus planos de treino aqui. Depois, vá para a aba "Registrar
                            " para lançar seus resultados."
                        }
                        button { class: "btn btn-primary",
                            // onclick: "showCreateWorkoutModal()",
                            "+ Criar Novo Plano de Treino"
                        }
                        div { id: "workoutsList", class: "workout-list",
                            div { class: "empty-state",
                                p { "Você ainda não tem planos de treino. Crie o primeiro!" }
                            }
                        }
                    }
                }

                div { id: "progress", class: "tab-content",
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

                div { id: "stats", class: "tab-content",
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
                button { class: "btn btn-primary",
                    // onclick: "showCreateWorkoutModal()",
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

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div { id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p {
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }

            // Navigation links
            Link { to: Route::Blog { id: id - 1 }, "Previous" }
            span { " <---> " }
            Link { to: Route::Blog { id: id + 1 }, "Next" }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Blog { id: 1 }, "Blog" }
            Link { to: Route::Workouts {}, "Workouts" }
        }

        Outlet::<Route> {}
    }
}
