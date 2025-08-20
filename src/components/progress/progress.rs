use dioxus::prelude::*;

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
