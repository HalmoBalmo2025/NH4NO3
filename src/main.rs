use dioxus::prelude::*;
use good_lp::*;
use anyhow::Result;

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

#[derive(Debug, Clone)]
struct Salt {
    name: &'static str,
    // macronutrients / major ions (mass fraction, g per g salt)
    nh4: f64,
    no3: f64,
    p: f64,   // elemental P (not PO₄³⁻)
    k: f64,
    ca: f64,
    mg: f64,
    s: f64,   // elemental S (not SO₄²⁻)
    cl: f64,
}

#[derive(Debug, Clone)]
struct OptimizationResult {
    recipe: Vec<(String, f64)>,
    nh4_actual: f64,
    no3_actual: f64,
    k_actual: f64,
    p_actual: f64,
    ca_actual: f64,
    mg_actual: f64,
    s_actual: f64,
    cl_actual: f64,
}

#[derive(Debug, Clone)]
struct ComparisonEntry {
    result: OptimizationResult,
    timestamp: String,
}

#[component]
fn FertilizerOptimizer() -> Element {
    let mut min_n = use_signal(|| 40.0);
    let mut min_n_focused = use_signal(|| false);
    let mut max_n = use_signal(|| 40.0);
    let mut max_n_focused = use_signal(|| false);
    let mut nh4_percentage = use_signal(|| 50.0);
    let mut no3_percentage = use_signal(|| 50.0);
    let mut nh4_focused = use_signal(|| false);
    let mut no3_focused = use_signal(|| false);
    let nh4_ratio = move || nh4_percentage() / 100.0;
    let mut min_k = use_signal(|| 15.0);
    let mut min_k_focused = use_signal(|| false);
    let mut max_k = use_signal(|| 25.0);
    let mut max_k_focused = use_signal(|| false);
    let mut min_p = use_signal(|| 4.0);
    let mut min_p_focused = use_signal(|| false);
    let mut max_p = use_signal(|| 8.0);
    let mut max_p_focused = use_signal(|| false);
    let mut min_ca = use_signal(|| 10.0);
    let mut min_ca_focused = use_signal(|| false);
    let mut max_ca = use_signal(|| 15.0);
    let mut max_ca_focused = use_signal(|| false);
    let mut min_mg = use_signal(|| 4.0);
    let mut min_mg_focused = use_signal(|| false);
    let mut max_mg = use_signal(|| 5.0);
    let mut max_mg_focused = use_signal(|| false);
    let mut min_s = use_signal(|| 20.0);
    let mut min_s_focused = use_signal(|| false);
    let mut max_s = use_signal(|| 25.0);
    let mut max_s_focused = use_signal(|| false);
    let mut min_cl = use_signal(|| 0.0);
    let mut min_cl_focused = use_signal(|| false);
    let mut max_cl = use_signal(|| 75.0);
    let mut max_cl_focused = use_signal(|| false);
    let mut result = use_signal(|| None::<OptimizationResult>);
    let mut current_result = use_signal(|| None::<OptimizationResult>);
    let mut error_msg = use_signal(|| None::<String>);
    let mut comparison_history = use_signal(|| Vec::<ComparisonEntry>::new());

    let salts: Vec<Salt> = vec![
        Salt { name: "Ca(NO₃)₂·4H₂O", nh4: 0.0, no3: 0.525133, p: 0.0, k: 0.0, ca: 0.169717, mg: 0.0, s: 0.0, cl: 0.0},
        Salt { name: "Mg(NO₃)₂·6H₂O", nh4: 0.0, no3: 0.483645, p: 0.0, k: 0.0, ca: 0.0, mg: 0.094792, s: 0.0, cl: 0.0},
        Salt { name: "KNO₃", nh4: 0.0, no3: 0.613282, p: 0.0, k: 0.386718, ca: 0.0, mg: 0.0, s: 0.0, cl: 0.0},
        Salt { name: "(NH₄)₂SO₄", nh4: 0.273031, no3: 0.0, p: 0.0, k: 0.0, ca: 0.0, mg: 0.0, s: 0.242661, cl: 0.0},
        Salt { name: "NH₄H₂PO₄", nh4: 0.156827, no3: 0.0, p: 0.269281, k: 0.0, ca: 0.0, mg: 0.0, s: 0.0, cl: 0.0},
        Salt { name: "NH₄Cl", nh4: 0.337247, no3: 0.0, p: 0.0, k: 0.0, ca: 0.0, mg: 0.0, s: 0.0, cl: 0.662753},
        Salt { name: "KH₂PO₄", nh4: 0.0, no3: 0.0, p: 0.227609, k: 0.287308, ca: 0.0, mg: 0.0, s: 0.0, cl: 0.0},
        Salt { name: "K₂SO₄", nh4: 0.0, no3: 0.0, p: 0.0, k: 0.448740, ca: 0.0, mg: 0.0, s: 0.184010, cl: 0.0},
        Salt { name: "MgSO₄·7H₂O", nh4: 0.0, no3: 0.0, p: 0.0, k: 0.0, ca: 0.0, mg: 0.098612, s: 0.130096, cl: 0.0},
        Salt { name: "CaCl₂·2H₂O", nh4: 0.0, no3: 0.0, p: 0.0, k: 0.0, ca: 0.272625, mg: 0.0, s: 0.0, cl: 0.482287},
        Salt { name: "Ferty 10", nh4: 0.0, no3: 0.0, p: 0.0, k: 0.0, ca: 0.0, mg: 0.0, s: 0.0, cl: 0.0},
        Salt { name: "Ferty 72", nh4: 0.0, no3: 0.0, p: 0.0, k: 0.0, ca: 0.0, mg: 0.0, s: 0.0, cl: 0.0},
    ];

    // Initial optimization on component mount
    use_effect({
        let salts = salts.clone();
        move || {
            let optimization_result = optimize_recipe(min_n(), max_n(), nh4_ratio(), min_k(), max_k(), min_p(), max_p(), min_ca(), max_ca(), min_mg(), max_mg(), min_s(), max_s(), min_cl(), max_cl(), false, &salts);
            match optimization_result {
                Ok(res) => {
                    result.set(Some(res.clone()));
                    current_result.set(Some(res));
                    error_msg.set(None);
                }
                Err(e) => {
                    error_msg.set(Some(format!("Nicht lösbar: {}", e)));
                }
            }
        }
    });

    // Real-time optimization when parameters change
    use_effect({
        let salts = salts.clone();
        move || {
            // This effect runs when any of these values change
            let _deps = (min_n(), max_n(), nh4_percentage(), min_k(), max_k(), min_p(), max_p(), min_ca(), max_ca(), min_mg(), max_mg(), min_s(), max_s(), min_cl(), max_cl());
            
            let optimization_result = optimize_recipe(min_n(), max_n(), nh4_ratio(), min_k(), max_k(), min_p(), max_p(), min_ca(), max_ca(), min_mg(), max_mg(), min_s(), max_s(), min_cl(), max_cl(), false, &salts);
            match optimization_result {
                Ok(res) => {
                    current_result.set(Some(res));
                    error_msg.set(None);
                }
                Err(e) => {
                    error_msg.set(Some(format!("Nicht lösbar: {}", e)));
                }
            }
        }
    });

    let fine_tune = {
        let salts = salts.clone();
        move |_| {
            let optimization_result = optimize_recipe(min_n(), max_n(), nh4_ratio(), min_k(), max_k(), min_p(), max_p(), min_ca(), max_ca(), min_mg(), max_mg(), min_s(), max_s(), min_cl(), max_cl(), true, &salts);
            match optimization_result {
                Ok(res) => {
                    result.set(Some(res.clone()));
                    current_result.set(Some(res));
                    error_msg.set(None);
                }
                Err(e) => {
                    error_msg.set(Some(format!("Feinabstimmung fehlgeschlagen: {}", e)));
                }
            }
        }
    };

    let save_recipe = move |_| {
        if let Some(res) = current_result() {
            let entry = ComparisonEntry {
                result: res.clone(),
                timestamp: format!("{:.0}% NH₄⁺", nh4_percentage()),
            };
            comparison_history.with_mut(|history| {
                history.push(entry);
            });
            result.set(Some(res));
        }
    };

    let clear_history = move |_| {
        comparison_history.set(Vec::new());
    };

    rsx! {
        div { class: "container",
            header { class: "header",
                h1 { "🧪 Nährlösungs-Rezeptur-Optimierer" }
                div { class: "description",
                    p { class: "subtitle-main", 
                        "Diese Website löst ein mathematisches Optimierungsproblem mittels linearer Programmierung. Der good_lp-Algorithmus berechnet die minimale Salzmasse, die erforderlich ist, um definierte Nährstoffkonzentrationen zu erreichen. Dabei werden die Massenbilanzgleichungen aller Makronährstoffe (NH₄⁺, NO₃⁻, K, P, Ca, Mg, S, Cl) als Nebenbedingungen berücksichtigt."
                    }
                    p { class: "subtitle-usage", 
                        "Die Parameter können links eingestellt werden, wobei die Berechnung in Echtzeit erfolgt. Die 'Optimale Rezeptur' zeigt die berechneten Salzmengen in g/L für Stammlösungen A und B. Der 'Vergleich der Nährlösungs-Rezepturen' dokumentiert mittels ‘Rezeptur speichern’ verschiedene NH₄⁺-Anteile mit den resultierenden Nährstoffkonzentrationen und ermöglicht den direkten Vergleich gespeicherter Rezepturen."
                    }
                    p { class: "subtitle-demo", 
                        "Klicken Sie in das NH₄⁺-Feld und nutzen die Pfeiltasten ↑/↓, um zu beobachten, wie sich die 'Optimale Rezeptur' und der 'Vergleich der Nährlösungs-Rezepturen' in Echtzeit verändern."
                    }
                    p { class: "subtitle-error", 
                        "Falls eine 'Nicht lösbar'-Meldung erscheint, sind die gewählten Parameterbereiche mathematisch unvereinbar - die verfügbaren Salze können die Zielkonzentrationen nicht gleichzeitig erfüllen."
                    }
                }
            }

            div { class: "main-layout",
                // Left column - Parameters
                div { class: "left-column",
                    div { class: "input-section",
                        h2 { "Parameter" }
                        
                        div { class: "input-group",
                            label { "Stickstoff-Bereich (g·l⁻¹)" }
                            div { class: "range-inputs",
                                div { class: "range-field",
                                    label { r#for: "min-n", "Min" }
                                    input {
                                        id: "min-n",
                                        r#type: "number",
                                        step: "0.1",
                                        min: "40.0",
                                        max: "40.0",
                                        value: if min_n_focused() { "{min_n}" } else { "" },
                                        placeholder: "{min_n}",
                                        onfocus: move |_| {
                                            min_n_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            min_n_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                min_n.set(val);
                                            }
                                        }
                                    }
                                }
                                div { class: "range-field",
                                    label { r#for: "max-n", "Max" }
                                    input {
                                        id: "max-n",
                                        r#type: "number",
                                        step: "0.1",
                                        min: "40.0",
                                        max: "40.0",
                                        value: if max_n_focused() { "{max_n}" } else { "" },
                                        placeholder: "{max_n}",
                                        onfocus: move |_| {
                                            max_n_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            max_n_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                max_n.set(val);
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "input-group",
                            label { "Stickstoff-Verhältnis (%)" }
                            div { class: "ratio-inputs",
                                div { class: "ratio-field",
                                    label { r#for: "nh4-percent", "NH₄⁺" }
                                    input {
                                        id: "nh4-percent",
                                        r#type: "number",
                                        step: "1",
                                        min: "0",
                                        max: "100",
                                        value: if nh4_focused() { "{nh4_percentage}" } else { "" },
                                        placeholder: "{nh4_percentage}",
                                        onfocus: move |_| {
                                            nh4_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            nh4_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                if val >= 0.0 && val <= 100.0 {
                                                    nh4_percentage.set(val);
                                                    no3_percentage.set(100.0 - val);
                                                }
                                            }
                                        }
                                    }
                                }
                                div { class: "ratio-field",
                                    label { r#for: "no3-percent", "NO₃⁻" }
                                    input {
                                        id: "no3-percent",
                                        r#type: "number",
                                        step: "1",
                                        min: "0",
                                        max: "100",
                                        value: if no3_focused() { "{no3_percentage}" } else { "" },
                                        placeholder: "{no3_percentage}",
                                        onfocus: move |_| {
                                            no3_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            no3_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                if val >= 0.0 && val <= 100.0 {
                                                    no3_percentage.set(val);
                                                    nh4_percentage.set(100.0 - val);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            small { "NH₄⁺- und NO₃⁻-Anteil am Gesamtstickstoff" }
                        }

                        div { class: "input-group",
                            label { "Kalium-Bereich (g·l⁻¹)" }
                            div { class: "range-inputs",
                                div { class: "range-field",
                                    label { r#for: "min-k", "Min" }
                                    input {
                                        id: "min-k",
                                        r#type: "number",
                                        step: "0.1",
                                        min: "15.0",
                                        max: "25.0",
                                        value: if min_k_focused() { "{min_k}" } else { "" },
                                        placeholder: "{min_k}",
                                        onfocus: move |_| {
                                            min_k_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            min_k_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                min_k.set(val);
                                            }
                                        }
                                    }
                                }
                                div { class: "range-field",
                                    label { r#for: "max-k", "Max" }
                                    input {
                                        id: "max-k",
                                        r#type: "number",
                                        step: "0.1",
                                        min: "15.0",
                                        max: "25.0",
                                        value: if max_k_focused() { "{max_k}" } else { "" },
                                        placeholder: "{max_k}",
                                        onfocus: move |_| {
                                            max_k_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            max_k_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                max_k.set(val);
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "input-group",
                            label { "Phosphor-Bereich (g·l⁻¹)" }
                            div { class: "range-inputs",
                                div { class: "range-field",
                                    label { r#for: "min-p", "Min" }
                                    input {
                                        id: "min-p",
                                        r#type: "number",
                                        step: "0.1",
                                        min: "4.0",
                                        max: "6.0",
                                        value: if min_p_focused() { "{min_p}" } else { "" },
                                        placeholder: "{min_p}",
                                        onfocus: move |_| {
                                            min_p_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            min_p_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                min_p.set(val);
                                            }
                                        }
                                    }
                                }
                                div { class: "range-field",
                                    label { r#for: "max-p", "Max" }
                                    input {
                                        id: "max-p",
                                        r#type: "number",
                                        step: "0.1",
                                        min: "4.0",
                                        max: "6.0",
                                        value: if max_p_focused() { "{max_p}" } else { "" },
                                        placeholder: "{max_p}",
                                        onfocus: move |_| {
                                            max_p_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            max_p_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                max_p.set(val);
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "input-group",
                            label { "Kalzium-Bereich (g·l⁻¹)" }
                            div { class: "range-inputs",
                                div { class: "range-field",
                                    label { r#for: "min-ca", "Min" }
                                    input {
                                        id: "min-ca",
                                        r#type: "number",
                                        step: "0.1",
                                        min: "9.0",
                                        max: "15.0",
                                        value: if min_ca_focused() { "{min_ca}" } else { "" },
                                        placeholder: "{min_ca}",
                                        onfocus: move |_| {
                                            min_ca_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            min_ca_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                min_ca.set(val);
                                            }
                                        }
                                    }
                                }
                                div { class: "range-field",
                                    label { r#for: "max-ca", "Max" }
                                    input {
                                        id: "max-ca",
                                        r#type: "number",
                                        step: "0.1",
                                        min: "9.0",
                                        max: "15.0",
                                        value: if max_ca_focused() { "{max_ca}" } else { "" },
                                        placeholder: "{max_ca}",
                                        onfocus: move |_| {
                                            max_ca_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            max_ca_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                max_ca.set(val);
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "input-group",
                            label { "Magnesium-Bereich (g·l⁻¹)" }
                            div { class: "range-inputs",
                                div { class: "range-field",
                                    label { r#for: "min-mg", "Min" }
                                    input {
                                        id: "min-mg",
                                        r#type: "number",
                                        step: "0.1",
                                        min: "3.5",
                                        max: "6.0",
                                        value: if min_mg_focused() { "{min_mg}" } else { "" },
                                        placeholder: "{min_mg}",
                                        onfocus: move |_| {
                                            min_mg_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            min_mg_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                min_mg.set(val);
                                            }
                                        }
                                    }
                                }
                                div { class: "range-field",
                                    label { r#for: "max-mg", "Max" }
                                    input {
                                        id: "max-mg",
                                        r#type: "number",
                                        step: "0.1",
                                        min: "3.5",
                                        max: "6.0",
                                        value: if max_mg_focused() { "{max_mg}" } else { "" },
                                        placeholder: "{max_mg}",
                                        onfocus: move |_| {
                                            max_mg_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            max_mg_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                max_mg.set(val);
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "input-group",
                            label { "Schwefel-Bereich (g·l⁻¹)" }
                            div { class: "range-inputs",
                                div { class: "range-field",
                                    label { r#for: "min-s", "Min" }
                                    input {
                                        id: "min-s",
                                        r#type: "number",
                                        step: "0.1",
                                        min: "15.0",
                                        max: "35.0",
                                        value: if min_s_focused() { "{min_s}" } else { "" },
                                        placeholder: "{min_s}",
                                        onfocus: move |_| {
                                            min_s_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            min_s_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                min_s.set(val);
                                            }
                                        }
                                    }
                                }
                                div { class: "range-field",
                                    label { r#for: "max-s", "Max" }
                                    input {
                                        id: "max-s",
                                        r#type: "number",
                                        step: "0.1",
                                        min: "15.0",
                                        max: "35.0",
                                        value: if max_s_focused() { "{max_s}" } else { "" },
                                        placeholder: "{max_s}",
                                        onfocus: move |_| {
                                            max_s_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            max_s_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                max_s.set(val);
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "input-group",
                            label { "Chlorid-Bereich (g·l⁻¹)" }
                            div { class: "range-inputs",
                                div { class: "range-field",
                                    label { r#for: "min-cl", "Min" }
                                    input {
                                        id: "min-cl",
                                        r#type: "number",
                                        step: "0.1",
                                        min: "0.0",
                                        max: "75.0",
                                        value: if min_cl_focused() { "{min_cl}" } else { "" },
                                        placeholder: "{min_cl}",
                                        onfocus: move |_| {
                                            min_cl_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            min_cl_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                min_cl.set(val);
                                            }
                                        }
                                    }
                                }
                                div { class: "range-field",
                                    label { r#for: "max-cl", "Max" }
                                    input {
                                        id: "max-cl",
                                        r#type: "number",
                                        step: "0.1",
                                        min: "0.0",
                                        max: "75.0",
                                        value: if max_cl_focused() { "{max_cl}" } else { "" },
                                        placeholder: "{max_cl}",
                                        onfocus: move |_| {
                                            max_cl_focused.set(true);
                                        },
                                        onblur: move |_| {
                                            max_cl_focused.set(false);
                                        },
                                        oninput: move |evt| {
                                            if let Ok(val) = evt.value().parse::<f64>() {
                                                max_cl.set(val);
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        button { class: "optimize-btn", onclick: fine_tune,
                            "🔬 Feinabstimmung"
                        }

                        if let Some(_) = result() {
                            button { class: "save-btn", onclick: save_recipe,
                                "💾 Rezeptur speichern"
                            }
                        }

                        if let Some(error) = error_msg() {
                            div { class: "error",
                                "⚠️ {error}"
                            }
                        }
                    }
                }

                // Right column - Results
                div { class: "right-column",
                    if let Some(res) = result() {
                        div { class: "results-section",
                            h2 { "Optimale Rezeptur" }
                            
                            div { class: "recipe-table",
                                h3 { "Nährsalz in Gramm pro 1 Liter Stammlösung" }
                                table {
                                    thead {
                                        tr {
                                            th { "Salz" }
                                            th { "SL A" }
                                            th { "SL B" }
                                        }
                                    }
                                    tbody {
                                        // First display SL A salts
                                        for (name, amount) in res.recipe.iter() {
                                            if name == "CaCl₂·2H₂O" || name == "Ca(NO₃)₂·4H₂O" || name == "Mg(NO₃)₂·6H₂O" {
                                                tr {
                                                    td { class: "salt-name", "{name}" }
                                                    td { class: "amount", "{amount:.2}" }
                                                    td { class: "amount", "—" }
                                                }
                                            }
                                        }
                                        // Always display Ferty 72 as last element of SL A
                                        tr {
                                            td { class: "salt-name", "Ferty 72" }
                                            td { class: "amount", "0.30" }
                                            td { class: "amount", "—" }
                                        }
                                        
                                        // Then display SL B salts
                                        for (name, amount) in res.recipe.iter() {
                                            if !(name == "CaCl₂·2H₂O" || name == "Ca(NO₃)₂·4H₂O" || name == "Mg(NO₃)₂·6H₂O" || name == "Ferty 72") {
                                                tr {
                                                    td { class: "salt-name", "{name}" }
                                                    td { class: "amount", "—" }
                                                    td { class: "amount", "{amount:.2}" }
                                                }
                                            }
                                        }
                                        // Always display Ferty 10 as last element of SL B
                                        tr {
                                            td { class: "salt-name", "Ferty 10" }
                                            td { class: "amount", "—" }
                                            td { class: "amount", "2.24" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Comparison table - always visible
                    div { class: "comparison-section",
                        div { class: "comparison-header",
                            h2 { "Vergleich der Nährlösungs-Rezepturen" }
                            if !comparison_history().is_empty() {
                                button { class: "clear-btn", onclick: clear_history,
                                    "Verlauf löschen"
                                }
                            }
                        }
                        
                        div { class: "comparison-table",
                            table {
                                thead {
                                    tr {
                                        th { "NH₄⁺-Anteil" }
                                        th { "NH₄⁺ (g·l⁻¹)" }
                                        th { "NO₃⁻ (g·l⁻¹)" }
                                        th { "K⁺ (g·l⁻¹)" }
                                        th { "P (g·l⁻¹)" }
                                        th { "Ca²⁺ (g·l⁻¹)" }
                                        th { "Mg²⁺ (g·l⁻¹)" }
                                        th { "S (g·l⁻¹)" }
                                        th { "Cl⁻ (g·l⁻¹)" }
                                        th { "Status" }
                                    }
                                }
                                tbody {
                                    // Show saved recipes
                                    for entry in comparison_history().iter() {
                                        tr {
                                            td { class: "ratio-cell", "{entry.timestamp}" }
                                            td { class: "nutrient-cell nh4", "{entry.result.nh4_actual:.3}" }
                                            td { class: "nutrient-cell no3", "{entry.result.no3_actual:.3}" }
                                            td { class: "nutrient-cell k", "{entry.result.k_actual:.3}" }
                                            td { class: "nutrient-cell p", "{entry.result.p_actual:.3}" }
                                            td { class: "nutrient-cell ca", "{entry.result.ca_actual:.3}" }
                                            td { class: "nutrient-cell mg", "{entry.result.mg_actual:.3}" }
                                            td { class: "nutrient-cell s", "{entry.result.s_actual:.3}" }
                                            td { class: "nutrient-cell cl", "{entry.result.cl_actual:.3}" }
                                            td { class: "status-saved", "💾 Gespeichert" }
                                        }
                                    }
                                    // Show current live result
                                    if let Some(current) = current_result() {
                                        tr { class: "current-row",
                                            td { class: "ratio-cell current", "{nh4_percentage():.0} % NH₄⁺" }
                                            td { class: "nutrient-cell nh4", "{current.nh4_actual:.3}" }
                                            td { class: "nutrient-cell no3", "{current.no3_actual:.3}" }
                                            td { class: "nutrient-cell k", "{current.k_actual:.3}" }
                                            td { class: "nutrient-cell p", "{current.p_actual:.3}" }
                                            td { class: "nutrient-cell ca", "{current.ca_actual:.3}" }
                                            td { class: "nutrient-cell mg", "{current.mg_actual:.3}" }
                                            td { class: "nutrient-cell s", "{current.s_actual:.3}" }
                                            td { class: "nutrient-cell cl", "{current.cl_actual:.3}" }
                                            td { class: "status-live", "🔄 Live" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn optimize_recipe(min_n: f64, max_n: f64, nh4_ratio: f64, min_k: f64, max_k: f64, min_p: f64, max_p: f64, min_ca: f64, max_ca: f64, min_mg: f64, max_mg: f64, min_s: f64, max_s: f64, min_cl: f64, max_cl: f64, is_fine_tuning: bool, salts: &[Salt]) -> Result<OptimizationResult> {
    // Create variables using the good_lp API
    let mut vars = variables!();
    
    // Add one variable for each salt (amount in g/L)
    let salt_vars: Vec<Variable> = salts.iter()
        .map(|_| vars.add(variable().min(0.0)))
        .collect();

    // Build nutrient expressions for all nutrients
    let mut nh4_expr = Expression::from(0.0);
    let mut no3_expr = Expression::from(0.0);
    let mut k_expr = Expression::from(0.0);
    let mut p_expr = Expression::from(0.0);
    let mut ca_expr = Expression::from(0.0);
    let mut mg_expr = Expression::from(0.0);
    let mut s_expr = Expression::from(0.0);
    let mut cl_expr = Expression::from(0.0);
    
    for (i, salt) in salts.iter().enumerate() {
        if salt.nh4 != 0.0 {
            nh4_expr = nh4_expr + salt_vars[i] * salt.nh4;
        }
        if salt.no3 != 0.0 {
            no3_expr = no3_expr + salt_vars[i] * salt.no3;
        }
        if salt.k != 0.0 {
            k_expr = k_expr + salt_vars[i] * salt.k;
        }
        if salt.p != 0.0 {
            p_expr = p_expr + salt_vars[i] * salt.p;
        }
        if salt.ca != 0.0 {
            ca_expr = ca_expr + salt_vars[i] * salt.ca;
        }
        if salt.mg != 0.0 {
            mg_expr = mg_expr + salt_vars[i] * salt.mg;
        }
        if salt.s != 0.0 {
            s_expr = s_expr + salt_vars[i] * salt.s;
        }
        if salt.cl != 0.0 {
            cl_expr = cl_expr + salt_vars[i] * salt.cl;
        }
    }

    // Objective: minimize total salt mass
    let total_mass = salt_vars.iter()
        .fold(Expression::from(0.0), |acc, &var| acc + var);

    // Apply stricter constraints when fine-tuning
    let (adj_min_cl, adj_max_cl) = if is_fine_tuning {
        (min_cl, max_cl * 0.8) // Reduce max chloride by 20% when fine-tuning
    } else {
        (min_cl, max_cl)
    };

    // Create and solve the model with all nutrient constraints
    let total_n_expr = nh4_expr.clone() + no3_expr.clone();
    let solution = vars.minimise(total_mass)
        .using(microlp)
        .with(constraint!(total_n_expr.clone() >= min_n))
        .with(constraint!(total_n_expr.clone() <= max_n))
        .with(constraint!(nh4_expr.clone() == nh4_ratio * total_n_expr.clone()))
        .with(constraint!(k_expr.clone() >= min_k))
        .with(constraint!(k_expr.clone() <= max_k))
        .with(constraint!(p_expr.clone() >= min_p))
        .with(constraint!(p_expr.clone() <= max_p))
        .with(constraint!(ca_expr.clone() >= min_ca))
        .with(constraint!(ca_expr.clone() <= max_ca))
        .with(constraint!(mg_expr.clone() >= min_mg))
        .with(constraint!(mg_expr.clone() <= max_mg))
        .with(constraint!(s_expr.clone() >= min_s))
        .with(constraint!(s_expr.clone() <= max_s))
        .with(constraint!(cl_expr.clone() >= adj_min_cl))
        .with(constraint!(cl_expr.clone() <= adj_max_cl))
        .solve()?;

    // Collect results
    let mut recipe = Vec::new();
    for (i, salt) in salts.iter().enumerate() {
        let qty = solution.value(salt_vars[i]);
        if qty > 1e-6 {
            recipe.push((salt.name.to_string(), qty));
        }
    }

    let nh4_actual = solution.eval(&nh4_expr);
    let no3_actual = solution.eval(&no3_expr);
    let k_actual = solution.eval(&k_expr);
    let p_actual = solution.eval(&p_expr);
    let ca_actual = solution.eval(&ca_expr);
    let mg_actual = solution.eval(&mg_expr);
    let s_actual = solution.eval(&s_expr);
    let cl_actual = solution.eval(&cl_expr);

    Ok(OptimizationResult {
        recipe,
        nh4_actual,
        no3_actual,
        k_actual,
        p_actual,
        ca_actual,
        mg_actual,
        s_actual,
        cl_actual,
    })
}

