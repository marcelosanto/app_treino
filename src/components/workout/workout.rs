use dioxus::prelude::*;

use crate::{
    components::workout::{create_workout::CreateWorkoutModal, ListWorkout, ViewWorkout},
    models::Workoute,
};

#[component]
pub fn Workouts() -> Element {
    let show_modal = use_signal(|| false);
    let selected_workout = use_signal(|| None);
    let workoutes = use_context::<Signal<Vec<Workoute>>>();
    let mut show_modal_create_workout = use_signal(|| false);

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
                    onclick: move |_| show_modal_create_workout.set(true),
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

        if show_modal_create_workout() {
            CreateWorkoutModal { show_modal: show_modal_create_workout }
        }
    }
}
