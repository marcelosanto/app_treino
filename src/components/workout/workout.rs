use dioxus::prelude::*;

use crate::{
    components::workout::{ListWorkout, ViewWorkout},
    models::Workoute,
};

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
                            ViewWorkout { selected_workout, show_modal }
                        }
                    }
                }
            }
        }
    }
}
