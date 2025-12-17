use dioxus::prelude::*;

mod models;
mod data;
mod optimizer;
mod components;

use components::FertilizerOptimizer;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        FertilizerOptimizer {}
    }
}
