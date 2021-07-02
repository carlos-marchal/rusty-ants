#[derive(Copy, Clone, Debug)]
pub struct Parameters {
    // α
    pub trail_importance: f32,
    // β
    pub distance_importance: f32,
    // Q
    pub distance_constant: f32,
    // ρ
    pub trail_decay: f32,
    // NC_max
    pub max_cycles: usize,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            trail_importance: 1.0,
            distance_importance: 5.0,
            distance_constant: 100.0,
            trail_decay: 0.5,
            max_cycles: 500,
        }
    }
}
