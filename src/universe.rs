use crate::cities::{generate as generate_cities, City};
use crate::cycle::Cycle;
use crate::edges::Edges;

#[derive(Copy, Clone)]
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

pub struct Universe {
    pub cities: Vec<City>,
    pub edges: Edges,
    pub cycle_count: usize,
    pub params: UniverseParams,
}

impl Universe {
    pub fn new(n: usize, params: &UniverseParams) -> Self {
        let cities = generate_cities(n);
        let edges = Edges::new(&cities, &params);
        Self {
            cities,
            edges,
            cycle_count: 0,
            params: *params,
        }
    }

    pub fn cycle(&mut self) {
        let mut cycle = Cycle::new(&self.edges, &self.params);
        while cycle.time < self.cities.len() {
            cycle.tick();
        }
    }
}
