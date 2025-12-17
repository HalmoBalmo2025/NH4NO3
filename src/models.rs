/// Data structures for the fertilizer optimizer application

#[derive(Debug, Clone, PartialEq)]
pub struct Salt {
    pub name: String,
    pub formula: String,
    pub is_custom: bool,
    pub enabled: bool,
    pub stock_solution: String,  // "A", "B", "C", etc. or "Unassigned"
    // macronutrients / major ions (mass fraction, g per g salt)
    pub nh4: f64,
    pub no3: f64,
    pub p: f64,   // elemental P (not PO₄³⁻)
    pub k: f64,
    pub ca: f64,
    pub mg: f64,
    pub s: f64,   // elemental S (not SO₄²⁻)
    pub cl: f64,
    // micronutrients (mass fraction, g per g salt)
    pub fe: f64,  // Iron
    pub mn: f64,  // Manganese
    pub zn: f64,  // Zinc
    pub cu: f64,  // Copper
    pub b: f64,   // Boron
    pub mo: f64,  // Molybdenum
}

impl Salt {
    /// Create a new predefined salt with default stock solution assignment
    pub fn predefined(
        name: &str,
        formula: &str,
        stock_solution: &str,
        nh4: f64, no3: f64, p: f64, k: f64, ca: f64, mg: f64, s: f64, cl: f64,
        fe: f64, mn: f64, zn: f64, cu: f64, b: f64, mo: f64,
    ) -> Self {
        Salt {
            name: name.to_string(),
            formula: formula.to_string(),
            is_custom: false,
            enabled: true,
            stock_solution: stock_solution.to_string(),
            nh4, no3, p, k, ca, mg, s, cl,
            fe, mn, zn, cu, b, mo,
        }
    }

    /// Create a new custom salt
    pub fn custom(
        name: String,
        formula: String,
        nh4: f64, no3: f64, p: f64, k: f64, ca: f64, mg: f64, s: f64, cl: f64,
        fe: f64, mn: f64, zn: f64, cu: f64, b: f64, mo: f64,
    ) -> Self {
        Salt {
            name,
            formula,
            is_custom: true,
            enabled: true,
            stock_solution: "Unassigned".to_string(),
            nh4, no3, p, k, ca, mg, s, cl,
            fe, mn, zn, cu, b, mo,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OptimizationResult {
    pub recipe: Vec<(String, f64)>,
    pub nh4_actual: f64,
    pub no3_actual: f64,
    pub k_actual: f64,
    pub p_actual: f64,
    pub ca_actual: f64,
    pub mg_actual: f64,
    pub s_actual: f64,
    pub cl_actual: f64,
    pub fe_actual: f64,
    pub mn_actual: f64,
    pub zn_actual: f64,
    pub cu_actual: f64,
    pub b_actual: f64,
    pub mo_actual: f64,
}

#[derive(Debug, Clone)]
pub struct ComparisonEntry {
    pub result: OptimizationResult,
    pub timestamp: String,
}

