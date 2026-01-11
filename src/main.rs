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
        ImpressumView {}
    }
}

#[component]
fn ImpressumView() -> Element {
    rsx! {
        div {
            class: "impressum-container",
            h2 { "Impressum" }
            
            div {
                class: "impressum-section",
                p {
                    r#"Hinweis zur Impressumspflicht: 
Der Betreiber dieser Website weist darauf hin, dass es sich um eine rein private Homepage ohne jeglichen geschäftsmäßigen Bezug im Sinne von § 5 TMG handelt.
Die Inhalte dienen ausschließlich privaten Zwecken und enthalten keine journalistisch-redaktionell gestalteten Angebote, die zur Meinungsbildung beitragen.
Aus diesem Grund besteht für diese Website keine gesetzliche Verpflichtung zur Bereitstellung eines Impressums im Sinne des Telemediengesetzes oder des Medienstaatsvertrages.
Kontakt: andreas.halm1993@icloud.com"#
                }
            }
        }
    }
}