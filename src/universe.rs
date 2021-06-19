#[derive(Copy, Clone, Debug)]
pub struct UniverseParams {
    // α
    pub trail_importance: f64,
    // β
    pub distance_importance: f64,
    // Q
    pub distance_constant: f64,
    // ρ
    pub trail_decay: f64,
    // NC_max
    pub max_cycles: usize,
}

impl Default for UniverseParams {
    fn default() -> Self {
        Self {
            trail_importance: 1.0,
            distance_importance: 5.0,
            distance_constant: 10000.0,
            trail_decay: 0.5,
            max_cycles: 500,
        }
    }
}
