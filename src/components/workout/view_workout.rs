use dioxus::prelude::*;

use crate::models::Workoute;

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
