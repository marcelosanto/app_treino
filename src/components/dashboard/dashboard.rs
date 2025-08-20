use dioxus::prelude::*;

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
