use dioxus::prelude::*;

use crate::components::home::home::Home;

mod components;
mod models;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    let config = dioxus::desktop::Config::new().with_menu(None);

    LaunchBuilder::desktop().with_cfg(config).launch(app);

    //dioxus::launch(App);
}

#[component]
fn app() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        style { {include_str!("../assets/main.css")} }
        Home {}
    }
}
