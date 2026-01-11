/// Main fertilizer optimizer component

use dioxus::prelude::*;
use crate::models::{ComparisonEntry, OptimizationResult};
use crate::optimizer::optimize_recipe;
use crate::data::get_predefined_salts;
use crate::components::UnifiedSaltManager;

#[component]
pub fn FertilizerOptimizer() -> Element {
    // Macronutrient parameters (g/L)
    let min_n = use_signal(|| 40.0);
    let max_n = use_signal(|| 40.0);
    let nh4_percentage = use_signal(|| 50.0);
    let no3_percentage = use_signal(|| 50.0);
    let nh4_ratio = move || nh4_percentage() / 100.0;
    let min_k = use_signal(|| 15.0);
    let max_k = use_signal(|| 25.0);
    let min_p = use_signal(|| 4.0);
    let max_p = use_signal(|| 8.0);
    let min_ca = use_signal(|| 10.0);
    let max_ca = use_signal(|| 15.0);
    let min_mg = use_signal(|| 4.0);
    let max_mg = use_signal(|| 5.0);
    let min_s = use_signal(|| 20.0);
    let max_s = use_signal(|| 25.0);
    let min_cl = use_signal(|| 0.0);
    let max_cl = use_signal(|| 75.0);
    
    // Micronutrient parameters (mg/L)
    let min_fe = use_signal(|| 0.0);
    let max_fe = use_signal(|| 5.0);
    let min_mn = use_signal(|| 0.0);
    let max_mn = use_signal(|| 2.0);
    let min_zn = use_signal(|| 0.0);
    let max_zn = use_signal(|| 0.5);
    let min_cu = use_signal(|| 0.0);
    let max_cu = use_signal(|| 0.2);
    let min_b = use_signal(|| 0.0);
    let max_b = use_signal(|| 0.5);
    let min_mo = use_signal(|| 0.0);
    let max_mo = use_signal(|| 0.1);
    
    // UI state
    let mut show_salt_manager = use_signal(|| false);
    let mut show_micronutrients = use_signal(|| false);
    
    // Salt and stock solution management
    let salts = use_signal(|| get_predefined_salts());
    let stock_solutions = use_signal(|| vec!["A".to_string(), "B".to_string()]);
    
    // Results
    let mut result = use_signal(|| None::<OptimizationResult>);
    let mut current_result = use_signal(|| None::<OptimizationResult>);
    let mut error_msg = use_signal(|| None::<String>);
    let mut comparison_history = use_signal(|| Vec::<ComparisonEntry>::new());

    // Initial optimization on component mount
    use_effect(move || {
        let enabled_salts: Vec<_> = salts().into_iter().filter(|s| s.enabled).collect();
        if enabled_salts.is_empty() {
            error_msg.set(Some("Keine Salze ausgewählt".to_string()));
            return;
        }
        
        let optimization_result = optimize_recipe(
            min_n(), max_n(), nh4_ratio(),
            min_k(), max_k(), min_p(), max_p(),
            min_ca(), max_ca(), min_mg(), max_mg(),
            min_s(), max_s(), min_cl(), max_cl(),
            min_fe(), max_fe(), min_mn(), max_mn(),
            min_zn(), max_zn(), min_cu(), max_cu(),
            min_b(), max_b(), min_mo(), max_mo(),
            false, &enabled_salts
        );
        
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
    });

    // Real-time optimization when parameters change
    use_effect(move || {
        let _deps = (
            min_n(), max_n(), nh4_percentage(),
            min_k(), max_k(), min_p(), max_p(),
            min_ca(), max_ca(), min_mg(), max_mg(),
            min_s(), max_s(), min_cl(), max_cl(),
            min_fe(), max_fe(), min_mn(), max_mn(),
            min_zn(), max_zn(), min_cu(), max_cu(),
            min_b(), max_b(), min_mo(), max_mo(),
            salts().len() // Trigger on salt changes
        );
        
        let enabled_salts: Vec<_> = salts().into_iter().filter(|s| s.enabled).collect();
        if enabled_salts.is_empty() {
            error_msg.set(Some("Keine Salze ausgewählt".to_string()));
            return;
        }
        
        let optimization_result = optimize_recipe(
            min_n(), max_n(), nh4_ratio(),
            min_k(), max_k(), min_p(), max_p(),
            min_ca(), max_ca(), min_mg(), max_mg(),
            min_s(), max_s(), min_cl(), max_cl(),
            min_fe(), max_fe(), min_mn(), max_mn(),
            min_zn(), max_zn(), min_cu(), max_cu(),
            min_b(), max_b(), min_mo(), max_mo(),
            false, &enabled_salts
        );
        
        match optimization_result {
            Ok(res) => {
                current_result.set(Some(res));
                error_msg.set(None);
            }
            Err(e) => {
                error_msg.set(Some(format!("Nicht lösbar: {}", e)));
            }
        }
    });

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
                    p { class: "subtitle-usage",
                        "Neu: Neben den Makronährstoffen werden nun auch Mikronährstoffe in der Optimierung berücksichtigt. Zudem haben Sie die Möglichkeit, eigene Nährsalze hinzuzufügen und die Datenbank individuell zu erweitern."
                    }
                }
            }


            // Toolbar
            div { class: "toolbar",
                button { 
                    class: "toolbar-btn",
                    onclick: move |_| show_salt_manager.set(!show_salt_manager()),
                    if show_salt_manager() { "✓ Nährsalz & Stammlösungen" } else { "Nährsalz & Stammlösungen" }
                }
            }

            // Unified Salt & Stock Solution Manager
            if show_salt_manager() {
                UnifiedSaltManager { salts: salts, stock_solutions: stock_solutions }
            }

            // Error Banner - Prominent and non-blocking
            if let Some(error) = error_msg() {
                div { class: "error-banner",
                    div { class: "error-banner-content",
                        div { class: "error-icon", "⚠️" }
                        div { class: "error-text",
                            div { class: "error-title", "Optimierung nicht möglich" }
                            div { class: "error-message", "{error}" }
                        }
                        div { class: "error-hint",
                            "💡 Tipp: Passen Sie die Nährstoffbereiche an oder aktivieren Sie zusätzliche Salze"
                        }
                    }
                }
            }

            div { class: "main-layout",
                // Left column - Parameters
                div { class: "left-column",
                    div { class: "input-section",
                        div { class: "section-header-with-toggle",
                            h2 { "Parameter" }
                            button { 
                                class: if show_micronutrients() { "micro-toggle-btn active" } else { "micro-toggle-btn" },
                                onclick: move |_| show_micronutrients.set(!show_micronutrients()),
                                if show_micronutrients() { "✓ Mikronährstoffe" } else { "Mikronährstoffe" }
                            }
                        }
                        
                        // Macronutrient inputs
                        {render_nutrient_input("Stickstoff (g l⁻¹)", "N", min_n, max_n)}
                        {render_nitrogen_ratio_input(nh4_percentage, no3_percentage)}
                        {render_nutrient_input("Kalium (g l⁻¹)", "K", min_k, max_k)}
                        {render_nutrient_input("Phosphor (g l⁻¹)", "P", min_p, max_p)}
                        {render_nutrient_input("Kalzium (g l⁻¹)", "Ca", min_ca, max_ca)}
                        {render_nutrient_input("Magnesium (g l⁻¹)", "Mg", min_mg, max_mg)}
                        {render_nutrient_input("Schwefel (g l⁻¹)", "S", min_s, max_s)}
                        {render_nutrient_input("Chlorid (g l⁻¹)", "Cl", min_cl, max_cl)}
                        
                        // Micronutrient inputs (collapsible)
                        if show_micronutrients() {
                            h3 { "Mikronährstoffe (mg l⁻¹)" }
                            {render_nutrient_input("Eisen (mg l⁻¹)", "Fe", min_fe, max_fe)}
                            {render_nutrient_input("Mangan (mg l⁻¹)", "Mn", min_mn, max_mn)}
                            {render_nutrient_input("Zink (mg l⁻¹)", "Zn", min_zn, max_zn)}
                            {render_nutrient_input("Kupfer (mg l⁻¹)", "Cu", min_cu, max_cu)}
                            {render_nutrient_input("Bor (mg l⁻¹)", "B", min_b, max_b)}
                            {render_nutrient_input("Molybdän (mg l⁻¹)", "Mo", min_mo, max_mo)}
                        }

                        if let Some(_) = result() {
                            button { class: "save-btn", onclick: save_recipe,
                                "💾 Rezeptur speichern"
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
                                            // Dynamically create column headers for each stock solution
                                            for solution in stock_solutions().iter() {
                                                th { key: "{solution}", "SL {solution}" }
                                            }
                                        }
                                    }
                                    tbody {
                                        // Display all salts from the recipe, grouped by stock solution
                                        for solution in stock_solutions().iter() {
                                            for (name, amount) in res.recipe.iter() {
                                                // Find the salt to check its stock solution assignment
                                                if let Some(salt) = salts().iter().find(|s| s.name == *name) {
                                                    if salt.stock_solution == *solution {
                                                        tr { key: "{solution}-{name}",
                                                            td { class: "salt-name", "{name}" }
                                                            // Create cells for each solution
                                                            for sol in stock_solutions().iter() {
                                                                if sol == solution {
                                                                    td { class: "amount", "{amount:.2}" }
                                                                } else {
                                                                    td { class: "amount", "—" }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            
                                            // Add fixed micronutrient mixes if they're assigned to this solution
                                            if solution == "A" && salts().iter().any(|s| s.name == "Ferty 72" && s.enabled && s.stock_solution == "A") {
                                                tr { key: "{solution}-ferty72",
                                                    td { class: "salt-name", "Ferty 72" }
                                                    for sol in stock_solutions().iter() {
                                                        if sol == "A" {
                                                            td { class: "amount", "0.30" }
                                                        } else {
                                                            td { class: "amount", "—" }
                                                        }
                                                    }
                                                }
                                            }
                                            if solution == "B" && salts().iter().any(|s| s.name == "Ferty 10" && s.enabled && s.stock_solution == "B") {
                                                tr { key: "{solution}-ferty10",
                                                    td { class: "salt-name", "Ferty 10" }
                                                    for sol in stock_solutions().iter() {
                                                        if sol == "B" {
                                                            td { class: "amount", "2.24" }
                                                        } else {
                                                            td { class: "amount", "—" }
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

                    // Comparison table
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
                                        th { "NH₄⁺ (g l⁻¹)" }
                                        th { "NO₃⁻ (g l⁻¹)" }
                                        th { "K⁺ (g l⁻¹)" }
                                        th { "P (g l⁻¹)" }
                                        th { "Ca²⁺ (g l⁻¹)" }
                                        th { "Mg²⁺ (g l⁻¹)" }
                                        th { "S (g l⁻¹)" }
                                        th { "Cl⁻ (g l⁻¹)" }
                                        if show_micronutrients() {
                                            th { "Fe (mg l⁻¹)" }
                                            th { "Mn (mg l⁻¹)" }
                                            th { "Zn (mg l⁻¹)" }
                                            th { "Cu (mg l⁻¹)" }
                                            th { "B (mg l⁻¹)" }
                                            th { "Mo (mg l⁻¹)" }
                                        }
                                        th { "Status" }
                                    }
                                }
                                tbody {
                                    // Show saved recipes
                                    for entry in comparison_history().iter() {
                                        tr {
                                            td { class: "ratio-cell", "{entry.timestamp}" }
                                            td { class: "nutrient-cell nh4", "{format_value(entry.result.nh4_actual)}" }
                                            td { class: "nutrient-cell no3", "{format_value(entry.result.no3_actual)}" }
                                            td { class: "nutrient-cell k", "{format_value(entry.result.k_actual)}" }
                                            td { class: "nutrient-cell p", "{format_value(entry.result.p_actual)}" }
                                            td { class: "nutrient-cell ca", "{format_value(entry.result.ca_actual)}" }
                                            td { class: "nutrient-cell mg", "{format_value(entry.result.mg_actual)}" }
                                            td { class: "nutrient-cell s", "{format_value(entry.result.s_actual)}" }
                                            td { class: "nutrient-cell cl", "{format_value(entry.result.cl_actual)}" }
                                            if show_micronutrients() {
                                                td { class: "nutrient-cell fe", "{format_value(entry.result.fe_actual)}" }
                                                td { class: "nutrient-cell mn", "{format_value(entry.result.mn_actual)}" }
                                                td { class: "nutrient-cell zn", "{format_value(entry.result.zn_actual)}" }
                                                td { class: "nutrient-cell cu", "{format_value(entry.result.cu_actual)}" }
                                                td { class: "nutrient-cell b", "{format_value(entry.result.b_actual)}" }
                                                td { class: "nutrient-cell mo", "{format_value(entry.result.mo_actual)}" }
                                            }
                                            td { class: "status-saved", "💾 Gespeichert" }
                                        }
                                    }
                                    // Show current live result
                                    if let Some(current) = current_result() {
                                        tr { class: "current-row",
                                            td { class: "ratio-cell current", "{nh4_percentage():.0} % NH₄⁺" }
                                            td { class: "nutrient-cell nh4", "{format_value(current.nh4_actual)}" }
                                            td { class: "nutrient-cell no3", "{format_value(current.no3_actual)}" }
                                            td { class: "nutrient-cell k", "{format_value(current.k_actual)}" }
                                            td { class: "nutrient-cell p", "{format_value(current.p_actual)}" }
                                            td { class: "nutrient-cell ca", "{format_value(current.ca_actual)}" }
                                            td { class: "nutrient-cell mg", "{format_value(current.mg_actual)}" }
                                            td { class: "nutrient-cell s", "{format_value(current.s_actual)}" }
                                            td { class: "nutrient-cell cl", "{format_value(current.cl_actual)}" }
                                            if show_micronutrients() {
                                                td { class: "nutrient-cell fe", "{format_value(current.fe_actual)}" }
                                                td { class: "nutrient-cell mn", "{format_value(current.mn_actual)}" }
                                                td { class: "nutrient-cell zn", "{format_value(current.zn_actual)}" }
                                                td { class: "nutrient-cell cu", "{format_value(current.cu_actual)}" }
                                                td { class: "nutrient-cell b", "{format_value(current.b_actual)}" }
                                                td { class: "nutrient-cell mo", "{format_value(current.mo_actual)}" }
                                            }
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

// Helper function to render a nutrient input group
fn render_nutrient_input(label: &str, _symbol: &str, mut min: Signal<f64>, mut max: Signal<f64>) -> Element {
    rsx! {
        div { class: "input-group",
            label { "{label}" }
            div { class: "range-inputs",
                div { class: "range-field",
                    label { "Min" }
                    input {
                        r#type: "number",
                        step: "0.1",
                        value: "{min}",
                        oninput: move |evt| {
                            if let Ok(val) = evt.value().parse::<f64>() {
                                min.set(val);
                            }
                        }
                    }
                }
                div { class: "range-field",
                    label { "Max" }
                    input {
                        r#type: "number",
                        step: "0.1",
                        value: "{max}",
                        oninput: move |evt| {
                            if let Ok(val) = evt.value().parse::<f64>() {
                                max.set(val);
                            }
                        }
                    }
                }
            }
        }
    }
}

// Helper function to format numbers, removing -0.000
fn format_value(val: f64) -> String {
    if val.abs() < 0.0001 {
        "0.000".to_string()
    } else {
        format!("{:.3}", val)
    }
}

// Helper function to render nitrogen ratio input
fn render_nitrogen_ratio_input(mut nh4: Signal<f64>, mut no3: Signal<f64>) -> Element {
    rsx! {
        div { class: "input-group",
            label { "Stickstoff-Verhältnis (%)" }
            div { class: "ratio-inputs",
                div { class: "ratio-field",
                    label { "NH₄⁺" }
                    input {
                        r#type: "number",
                        step: "1",
                        min: "0",
                        max: "100",
                        value: "{nh4}",
                        oninput: move |evt| {
                            if let Ok(val) = evt.value().parse::<f64>() {
                                if val >= 0.0 && val <= 100.0 {
                                    nh4.set(val);
                                    no3.set(100.0 - val);
                                }
                            }
                        }
                    }
                }
                div { class: "ratio-field",
                    label { "NO₃⁻" }
                    input {
                        r#type: "number",
                        step: "1",
                        min: "0",
                        max: "100",
                        value: "{no3}",
                        oninput: move |evt| {
                            if let Ok(val) = evt.value().parse::<f64>() {
                                if val >= 0.0 && val <= 100.0 {
                                    no3.set(val);
                                    nh4.set(100.0 - val);
                                }
                            }
                        }
                    }
                }
            }
            small { "NH₄⁺- und NO₃⁻-Anteil am Gesamtstickstoff" }
        }
    }
}
