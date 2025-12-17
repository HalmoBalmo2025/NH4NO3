/// Linear programming optimizer for fertilizer recipes

use anyhow::Result;
use good_lp::*;
use crate::models::{Salt, OptimizationResult};

/// Optimizes fertilizer recipe using linear programming
/// 
/// # Arguments
/// * `min_n`, `max_n` - Nitrogen range (g/L)
/// * `nh4_ratio` - Ratio of NH4+ to total nitrogen (0.0 to 1.0)
/// * `min_k`, `max_k` - Potassium range (g/L)
/// * `min_p`, `max_p` - Phosphorus range (g/L)
/// * `min_ca`, `max_ca` - Calcium range (g/L)
/// * `min_mg`, `max_mg` - Magnesium range (g/L)
/// * `min_s`, `max_s` - Sulfur range (g/L)
/// * `min_cl`, `max_cl` - Chloride range (g/L)
/// * Micronutrient ranges (mg/L): Fe, Mn, Zn, Cu, B, Mo
/// * `is_fine_tuning` - If true, applies stricter chloride constraints
/// * `salts` - Available salts with their nutrient compositions
#[allow(clippy::too_many_arguments)]
pub fn optimize_recipe(
    min_n: f64, 
    max_n: f64, 
    nh4_ratio: f64, 
    min_k: f64, 
    max_k: f64, 
    min_p: f64, 
    max_p: f64, 
    min_ca: f64, 
    max_ca: f64, 
    min_mg: f64, 
    max_mg: f64, 
    min_s: f64, 
    max_s: f64, 
    min_cl: f64, 
    max_cl: f64,
    min_fe: f64,
    max_fe: f64,
    min_mn: f64,
    max_mn: f64,
    min_zn: f64,
    max_zn: f64,
    min_cu: f64,
    max_cu: f64,
    min_b: f64,
    max_b: f64,
    min_mo: f64,
    max_mo: f64,
    is_fine_tuning: bool, 
    salts: &[Salt]
) -> Result<OptimizationResult> {
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
    let mut fe_expr = Expression::from(0.0);
    let mut mn_expr = Expression::from(0.0);
    let mut zn_expr = Expression::from(0.0);
    let mut cu_expr = Expression::from(0.0);
    let mut b_expr = Expression::from(0.0);
    let mut mo_expr = Expression::from(0.0);
    
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
        if salt.fe != 0.0 {
            fe_expr = fe_expr + salt_vars[i] * salt.fe;
        }
        if salt.mn != 0.0 {
            mn_expr = mn_expr + salt_vars[i] * salt.mn;
        }
        if salt.zn != 0.0 {
            zn_expr = zn_expr + salt_vars[i] * salt.zn;
        }
        if salt.cu != 0.0 {
            cu_expr = cu_expr + salt_vars[i] * salt.cu;
        }
        if salt.b != 0.0 {
            b_expr = b_expr + salt_vars[i] * salt.b;
        }
        if salt.mo != 0.0 {
            mo_expr = mo_expr + salt_vars[i] * salt.mo;
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
        // Macronutrient constraints
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
        // Micronutrient constraints (converted to g/L for consistency)
        .with(constraint!(fe_expr.clone() >= min_fe / 1000.0))
        .with(constraint!(fe_expr.clone() <= max_fe / 1000.0))
        .with(constraint!(mn_expr.clone() >= min_mn / 1000.0))
        .with(constraint!(mn_expr.clone() <= max_mn / 1000.0))
        .with(constraint!(zn_expr.clone() >= min_zn / 1000.0))
        .with(constraint!(zn_expr.clone() <= max_zn / 1000.0))
        .with(constraint!(cu_expr.clone() >= min_cu / 1000.0))
        .with(constraint!(cu_expr.clone() <= max_cu / 1000.0))
        .with(constraint!(b_expr.clone() >= min_b / 1000.0))
        .with(constraint!(b_expr.clone() <= max_b / 1000.0))
        .with(constraint!(mo_expr.clone() >= min_mo / 1000.0))
        .with(constraint!(mo_expr.clone() <= max_mo / 1000.0))
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
    let fe_actual = solution.eval(&fe_expr) * 1000.0; // Convert to mg/L
    let mn_actual = solution.eval(&mn_expr) * 1000.0;
    let zn_actual = solution.eval(&zn_expr) * 1000.0;
    let cu_actual = solution.eval(&cu_expr) * 1000.0;
    let b_actual = solution.eval(&b_expr) * 1000.0;
    let mo_actual = solution.eval(&mo_expr) * 1000.0;

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
        fe_actual,
        mn_actual,
        zn_actual,
        cu_actual,
        b_actual,
        mo_actual,
    })
}

