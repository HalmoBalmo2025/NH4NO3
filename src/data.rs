/// Salt database for fertilizer optimization

use crate::models::Salt;

/// Returns the list of predefined salts with their nutrient compositions
/// Note: Micronutrient values (Fe, Mn, Zn, Cu, B, Mo) are set to 0.0 for macronutrient salts
/// Users can add micronutrient-specific salts as custom entries
pub fn get_predefined_salts() -> Vec<Salt> {
    vec![
        // Stock Solution A - Calcium and Magnesium salts
        Salt::predefined("Ca(NO₃)₂·4H₂O", "Ca(NO₃)₂·4H₂O", "A",
            0.0142, 0.6375, 0.0, 0.0, 0.169717, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        Salt::predefined("Mg(NO₃)₂·6H₂O", "Mg(NO₃)₂·6H₂O", "A",
            0.0, 0.483645, 0.0, 0.0, 0.0, 0.094792, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        Salt::predefined("CaCl₂·2H₂O", "CaCl₂·2H₂O", "A",
            0.0, 0.0, 0.0, 0.0, 0.272625, 0.0, 0.0, 0.482287,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        Salt::predefined("Ferty 72", "Micronutrient Mix", "A",
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        
        // Stock Solution B - Phosphate and other salts
        Salt::predefined("KNO₃", "KNO₃", "B",
            0.0, 0.613282, 0.0, 0.386718, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        Salt::predefined("(NH₄)₂SO₄", "(NH₄)₂SO₄", "B",
            0.273031, 0.0, 0.0, 0.0, 0.0, 0.0, 0.242661, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        Salt::predefined("NH₄H₂PO₄", "NH₄H₂PO₄", "B",
            0.156827, 0.0, 0.269281, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        Salt::predefined("NH₄Cl", "NH₄Cl", "B",
            0.337247, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.662753,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        Salt::predefined("KH₂PO₄", "KH₂PO₄", "B",
            0.0, 0.0, 0.227609, 0.287308, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        Salt::predefined("K₂SO₄", "K₂SO₄", "B",
            0.0, 0.0, 0.0, 0.448740, 0.0, 0.0, 0.184010, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        Salt::predefined("MgSO₄·7H₂O", "MgSO₄·7H₂O", "B",
            0.0, 0.0, 0.0, 0.0, 0.0, 0.098612, 0.130096, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        Salt::predefined("Ferty 10", "Micronutrient Mix", "B",
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
    ]
}

