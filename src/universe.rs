use crate::cities::City;
use crate::cycle::{Cycle, CycleResult};
use crate::edges::Edges;

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
            distance_constant: 100.0,
            trail_decay: 0.5,
            max_cycles: 1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Universe {
    pub cities: Vec<City>,
    pub edges: Edges,
    pub cycle_count: usize,
    pub params: UniverseParams,
}

impl Universe {
    pub fn new(cities: &[City], params: &UniverseParams) -> Self {
        let edges = Edges::new(&cities, &params);
        Self {
            cities: cities.to_vec(),
            edges,
            cycle_count: 0,
            params: *params,
        }
    }

    pub fn cycle(&mut self) -> Option<CycleResult> {
        if self.cycle_count < self.params.max_cycles {
            let cycle = Cycle::new(&mut self.edges, &self.params);
            let result = cycle.complete();
            self.cycle_count += 1;
            Some(result)
        } else {
            None
        }
    }

    pub fn solve(mut self) -> CycleResult {
        let mut last_result: Option<CycleResult> = None;
        while let Some(result) = self.cycle() {
            last_result = Some(result);
        }
        last_result.unwrap()
    }
}
