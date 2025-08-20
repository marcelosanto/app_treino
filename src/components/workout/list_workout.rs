use dioxus::prelude::*;

use crate::models::Workoute;

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
