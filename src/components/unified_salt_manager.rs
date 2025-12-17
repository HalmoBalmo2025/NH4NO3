/// Unified salt and stock solution manager with modern tile design

use dioxus::prelude::*;
use crate::models::Salt;

/// Check if a stock solution has incompatible salt combinations (Ca/Mg + Phosphate)
fn check_incompatible_salts(salts: &[Salt], solution: &str) -> Option<(Vec<String>, Vec<String>)> {
    let solution_salts: Vec<&Salt> = salts.iter()
        .filter(|s| s.stock_solution == solution && s.enabled)
        .collect();
    
    let ca_mg_salts: Vec<String> = solution_salts.iter()
        .filter(|s| s.ca > 0.01 || s.mg > 0.01)
        .map(|s| s.name.clone())
        .collect();
    
    let phosphate_salts: Vec<String> = solution_salts.iter()
        .filter(|s| s.p > 0.01)
        .map(|s| s.name.clone())
        .collect();
    
    if !ca_mg_salts.is_empty() && !phosphate_salts.is_empty() {
        Some((ca_mg_salts, phosphate_salts))
    } else {
        None
    }
}

#[component]
pub fn UnifiedSaltManager(salts: Signal<Vec<Salt>>, stock_solutions: Signal<Vec<String>>) -> Element {
    let mut dragged_salt_idx = use_signal(|| None::<usize>);
    let mut show_add_form = use_signal(|| false);
    
    // Form state for adding custom salts
    let mut new_salt_name = use_signal(|| String::new());
    let mut new_salt_formula = use_signal(|| String::new());
    let mut new_nh4 = use_signal(|| String::from("0.0"));
    let mut new_no3 = use_signal(|| String::from("0.0"));
    let mut new_p = use_signal(|| String::from("0.0"));
    let mut new_k = use_signal(|| String::from("0.0"));
    let mut new_ca = use_signal(|| String::from("0.0"));
    let mut new_mg = use_signal(|| String::from("0.0"));
    let mut new_s = use_signal(|| String::from("0.0"));
    let mut new_cl = use_signal(|| String::from("0.0"));
    let mut new_fe = use_signal(|| String::from("0.0"));
    let mut new_mn = use_signal(|| String::from("0.0"));
    let mut new_zn = use_signal(|| String::from("0.0"));
    let mut new_cu = use_signal(|| String::from("0.0"));
    let mut new_b = use_signal(|| String::from("0.0"));
    let mut new_mo = use_signal(|| String::from("0.0"));

    let add_solution = move |_| {
        stock_solutions.with_mut(|solutions| {
            let next_letter = get_next_solution_letter(solutions);
            solutions.push(next_letter);
        });
    };

    let mut remove_solution = move |solution: String| {
        salts.with_mut(|s| {
            for salt in s.iter_mut() {
                if salt.stock_solution == solution {
                    salt.stock_solution = "Unassigned".to_string();
                }
            }
        });
        stock_solutions.with_mut(|solutions| {
            solutions.retain(|s| s != &solution);
        });
    };

    let mut toggle_salt = move |idx: usize| {
        salts.with_mut(|s| {
            if let Some(salt) = s.get_mut(idx) {
                salt.enabled = !salt.enabled;
            }
        });
    };

    let mut delete_salt = move |idx: usize| {
        salts.with_mut(|s| {
            s.remove(idx);
        });
    };

    let add_custom_salt = move |_| {
        let name = new_salt_name();
        let formula = new_salt_formula();
        
        if name.trim().is_empty() {
            return;
        }

        let nh4 = new_nh4().parse().unwrap_or(0.0);
        let no3 = new_no3().parse().unwrap_or(0.0);
        let p = new_p().parse().unwrap_or(0.0);
        let k = new_k().parse().unwrap_or(0.0);
        let ca = new_ca().parse().unwrap_or(0.0);
        let mg = new_mg().parse().unwrap_or(0.0);
        let s = new_s().parse().unwrap_or(0.0);
        let cl = new_cl().parse().unwrap_or(0.0);
        let fe = new_fe().parse().unwrap_or(0.0);
        let mn = new_mn().parse().unwrap_or(0.0);
        let zn = new_zn().parse().unwrap_or(0.0);
        let cu = new_cu().parse().unwrap_or(0.0);
        let b = new_b().parse().unwrap_or(0.0);
        let mo = new_mo().parse().unwrap_or(0.0);

        let custom_salt = Salt::custom(name, formula, nh4, no3, p, k, ca, mg, s, cl, fe, mn, zn, cu, b, mo);
        salts.with_mut(|s| s.push(custom_salt));

        // Reset form
        new_salt_name.set(String::new());
        new_salt_formula.set(String::new());
        new_nh4.set(String::from("0.0"));
        new_no3.set(String::from("0.0"));
        new_p.set(String::from("0.0"));
        new_k.set(String::from("0.0"));
        new_ca.set(String::from("0.0"));
        new_mg.set(String::from("0.0"));
        new_s.set(String::from("0.0"));
        new_cl.set(String::from("0.0"));
        new_fe.set(String::from("0.0"));
        new_mn.set(String::from("0.0"));
        new_zn.set(String::from("0.0"));
        new_cu.set(String::from("0.0"));
        new_b.set(String::from("0.0"));
        new_mo.set(String::from("0.0"));
        show_add_form.set(false);
    };

    rsx! {
        div { class: "unified-salt-manager",
            div { class: "manager-header",
                h2 { "🧪 Nährsalz & Stammlösungen" }
                p { class: "manager-instructions",
                    "Wählen Sie Ihre Nährsalze und ziehen Sie sie per Drag & Drop in die gewünschte Stammlösung. "
                    strong { "Wichtig: " }
                    "Lagern Sie Ca/Mg-Salze getrennt von Phosphat-Salzen, um Ausfällungen zu vermeiden."
                }
            }

            div { class: "solutions-layout",
                // Available salts section (left side / top on mobile)
                div { class: "available-salts-section",
                    div { class: "section-header",
                        h3 { "🧂 Verfügbare Nährsalze" }
                        button {
                            class: "add-salt-text-btn",
                            onclick: move |_| show_add_form.set(!show_add_form()),
                            title: "Benutzerdefiniertes Salz hinzufügen",
                            "Neues Nährsalz ➕"
                        }
                    }
                    
                    // Tile grid for unassigned/available salts
                    div { 
                        class: "salt-tiles-grid drop-zone",
                        ondrop: move |_| {
                            if let Some(idx) = dragged_salt_idx() {
                                salts.with_mut(|s| {
                                    if let Some(salt) = s.get_mut(idx) {
                                        salt.stock_solution = "Unassigned".to_string();
                                    }
                                });
                                dragged_salt_idx.set(None);
                            }
                        },
                        ondragover: move |evt| {
                            evt.prevent_default();
                        },
                        
                        for (idx, salt) in salts().iter().enumerate() {
                            if salt.stock_solution == "Unassigned" {
                                div {
                                    key: "{idx}",
                                    class: if salt.enabled { "salt-tile enabled" } else { "salt-tile disabled" },
                                    draggable: "true",
                                    ondragstart: move |_| {
                                        dragged_salt_idx.set(Some(idx));
                                    },
                                    ondragend: move |_| {
                                        dragged_salt_idx.set(None);
                                    },
                                    
                                    div { class: "salt-tile-content",
                                        div { class: "salt-icon", "🧂" }
                                        div { class: "salt-info",
                                            div { class: "salt-name", "{salt.name}" }
                                            div { class: "salt-formula", "{salt.formula}" }
                                        }
                                    }
                                    
                                    div { class: "salt-tile-actions",
                                        button {
                                            class: "tile-action-btn toggle",
                                            onclick: move |_| toggle_salt(idx),
                                            title: if salt.enabled { "Deaktivieren" } else { "Aktivieren" },
                                            if salt.enabled { "👁" } else { "👁‍🗨" }
                                        }
                                        if salt.is_custom {
                                            button {
                                                class: "tile-action-btn delete",
                                                onclick: move |_| delete_salt(idx),
                                                title: "Löschen",
                                                "🗑"
                                            }
                                        }
                                    }
                                    
                                    if !salt.enabled {
                                        div { class: "disabled-overlay" }
                                    }
                                }
                            }
                        }
                        
                        if salts().iter().filter(|s| s.stock_solution == "Unassigned").count() == 0 {
                            div { class: "empty-tiles-message",
                                "✨ Alle Salze sind Stammlösungen zugewiesen"
                            }
                        }
                    }
                    
                    // Add custom salt form
                    if show_add_form() {
                        div { class: "add-salt-form-compact",
                            h4 { "Neues Salz hinzufügen" }
                            div { class: "form-row-compact",
                                input {
                                    r#type: "text",
                                    placeholder: "Name (z.B. FeSO₄·7H₂O)",
                                    value: "{new_salt_name}",
                                    oninput: move |evt| new_salt_name.set(evt.value())
                                }
                                input {
                                    r#type: "text",
                                    placeholder: "Formel",
                                    value: "{new_salt_formula}",
                                    oninput: move |evt| new_salt_formula.set(evt.value())
                                }
                            }
                            details { class: "nutrient-details",
                                summary { "Nährstoffzusammensetzung eingeben" }
                                div { class: "compact-nutrient-grid",
                                    input { r#type: "number", placeholder: "NH₄⁺", value: "{new_nh4}", oninput: move |evt| new_nh4.set(evt.value()) }
                                    input { r#type: "number", placeholder: "NO₃⁻", value: "{new_no3}", oninput: move |evt| new_no3.set(evt.value()) }
                                    input { r#type: "number", placeholder: "P", value: "{new_p}", oninput: move |evt| new_p.set(evt.value()) }
                                    input { r#type: "number", placeholder: "K", value: "{new_k}", oninput: move |evt| new_k.set(evt.value()) }
                                    input { r#type: "number", placeholder: "Ca", value: "{new_ca}", oninput: move |evt| new_ca.set(evt.value()) }
                                    input { r#type: "number", placeholder: "Mg", value: "{new_mg}", oninput: move |evt| new_mg.set(evt.value()) }
                                    input { r#type: "number", placeholder: "S", value: "{new_s}", oninput: move |evt| new_s.set(evt.value()) }
                                    input { r#type: "number", placeholder: "Cl", value: "{new_cl}", oninput: move |evt| new_cl.set(evt.value()) }
                                    input { r#type: "number", placeholder: "Fe", value: "{new_fe}", oninput: move |evt| new_fe.set(evt.value()) }
                                    input { r#type: "number", placeholder: "Mn", value: "{new_mn}", oninput: move |evt| new_mn.set(evt.value()) }
                                    input { r#type: "number", placeholder: "Zn", value: "{new_zn}", oninput: move |evt| new_zn.set(evt.value()) }
                                    input { r#type: "number", placeholder: "Cu", value: "{new_cu}", oninput: move |evt| new_cu.set(evt.value()) }
                                    input { r#type: "number", placeholder: "B", value: "{new_b}", oninput: move |evt| new_b.set(evt.value()) }
                                    input { r#type: "number", placeholder: "Mo", value: "{new_mo}", oninput: move |evt| new_mo.set(evt.value()) }
                                }
                            }
                            div { class: "form-actions-compact",
                                button { class: "btn-add", onclick: add_custom_salt, "Hinzufügen" }
                                button { class: "btn-cancel", onclick: move |_| show_add_form.set(false), "Abbrechen" }
                            }
                        }
                    }
                }

                // Stock solutions section (right side / bottom on mobile)
                div { class: "stock-solutions-section",
                    for solution in stock_solutions().iter() {
                        {
                            let incompatible = check_incompatible_salts(&salts(), solution);
                            let has_warning = incompatible.is_some();
                            
                            rsx! {
                                div { 
                                    key: "{solution}",
                                    class: if has_warning { "solution-container warning" } else { "solution-container" },
                                    
                                    div { class: "solution-header-bar",
                                        h3 { "🧪 Stammlösung {solution}" }
                                        if stock_solutions().len() > 2 {
                                            button {
                                                class: "remove-sol-btn",
                                                onclick: {
                                                    let sol = solution.clone();
                                                    move |_| remove_solution(sol.clone())
                                                },
                                                title: "Stammlösung entfernen",
                                                "✕"
                                            }
                                        }
                                    }
                                    
                                    // Incompatibility warning
                                    if let Some((ca_mg, phosphates)) = incompatible {
                                        div { class: "incompatibility-warning",
                                            div { class: "warning-header",
                                                span { class: "warning-icon", "⚠️" }
                                                span { class: "warning-title", "Inkompatible Kombination!" }
                                            }
                                            div { class: "warning-message",
                                                "Ca/Mg-Salze reagieren mit Phosphat-Salzen und bilden unlösliche Ausfällungen."
                                            }
                                            div { class: "warning-details",
                                                div { class: "salt-list",
                                                    strong { "Ca/Mg: " }
                                                    span { "{ca_mg.join(\", \")}" }
                                                }
                                                div { class: "salt-list",
                                                    strong { "Phosphat: " }
                                                    span { "{phosphates.join(\", \")}" }
                                                }
                                            }
                                            div { class: "warning-action",
                                                "→ Verschieben Sie diese Salze in getrennte Stammlösungen"
                                            }
                                        }
                                    }
                                    
                                    div { 
                                class: "salt-tiles-grid drop-zone solution-drop",
                                ondrop: {
                                    let sol = solution.clone();
                                    move |_| {
                                        if let Some(idx) = dragged_salt_idx() {
                                            salts.with_mut(|s| {
                                                if let Some(salt) = s.get_mut(idx) {
                                                    salt.stock_solution = sol.clone();
                                                }
                                            });
                                            dragged_salt_idx.set(None);
                                        }
                                    }
                                },
                                ondragover: move |evt| {
                                    evt.prevent_default();
                                },
                                
                                for (idx, salt) in salts().iter().enumerate() {
                                    if salt.stock_solution == *solution {
                                        div {
                                            key: "{idx}",
                                            class: if salt.enabled { "salt-tile enabled in-solution" } else { "salt-tile disabled in-solution" },
                                            draggable: "true",
                                            ondragstart: move |_| {
                                                dragged_salt_idx.set(Some(idx));
                                            },
                                            ondragend: move |_| {
                                                dragged_salt_idx.set(None);
                                            },
                                            
                                            div { class: "salt-tile-content",
                                                div { class: "salt-icon", "🧂" }
                                                div { class: "salt-info",
                                                    div { class: "salt-name", "{salt.name}" }
                                                    div { class: "salt-formula", "{salt.formula}" }
                                                }
                                            }
                                            
                                            if !salt.enabled {
                                                div { class: "disabled-overlay" }
                                            }
                                        }
                                    }
                                }
                                
                                if salts().iter().filter(|s| s.stock_solution == *solution && s.enabled).count() == 0 {
                                    div { class: "empty-solution-hint",
                                        "← Ziehen Sie Salze hierher"
                                    }
                                }
                            }
                        }
                            }
                        }
                    }
                    
                    // Add solution button
                    if stock_solutions().len() < 10 {
                        button { 
                            class: "add-solution-card",
                            onclick: add_solution,
                            div { class: "add-solution-icon", "➕" }
                            div { class: "add-solution-text", "Neue Stammlösung" }
                        }
                    }
                }
            }
        }
    }
}

fn get_next_solution_letter(solutions: &[String]) -> String {
    let letters = "ABCDEFGHIJ";
    for letter in letters.chars() {
        let s = letter.to_string();
        if !solutions.contains(&s) {
            return s;
        }
    }
    "X".to_string()
}

