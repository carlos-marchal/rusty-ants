use crate::cities::{generate as generate_cities, City};
use crate::cycle::Cycle;
use crate::edges::Edges;

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

pub struct Universe<'a> {
    cities: Vec<City>,
    edges: Edges<'a>,
    cycle: Cycle<'a>,
    cycle_count: usize,
    params: UniverseParams,
}

impl<'a> Universe<'a> {
    pub fn new(n: usize, params: UniverseParams) -> Box<Universe<'a>> {
        let cities = generate_cities(n);
        let edges = Edges::new(&cities, &params);
        let cycle = Cycle::new(&cities, &edges, &params);
        Box::new(Universe {
            cities,
            edges,
            cycle,
            cycle_count: 0,
            params,
        })
    }
}
