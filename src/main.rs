use dioxus::prelude::*;

use crate::components::home::home::Home;

mod components;
mod models;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    // let config = dioxus::desktop::Config::new().with_menu(None);
    // LaunchBuilder::desktop().with_cfg(config).launch(app);

    //dioxus::launch(App);

    #[cfg(feature = "desktop")]
    {
        // Se a feature "desktop" estiver ativa, usamos a configuração de desktop.
        let config = dioxus::desktop::Config::new().with_menu(None);
        LaunchBuilder::new().with_cfg(config).launch(app);
    }

    #[cfg(feature = "mobile")]
    {
        // Se a feature "mobile" estiver ativa, usamos a configuração de mobile.
        let config = dioxus::mobile::Config::new();
        LaunchBuilder::new().with_cfg(config).launch(app);
    }

    // Se nenhuma feature de plataforma for especificada,
    // você pode ter um fallback ou um erro de compilação.
    #[cfg(not(any(feature = "desktop", feature = "mobile")))]
    compile_error!("Please enable either the 'desktop' or 'mobile' feature.");
}

#[component]
fn app() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        style { {include_str!("../assets/main.css")} }
        Home {}
    }
}
