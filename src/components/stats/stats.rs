use dioxus::prelude::*;

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
