use crate::cities::*;
use crate::parameters::Parameters;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    pub distance: f32,
    pub attractiveness: f32,
    pub total_trail: f32,
    pub recent_trail: f32,
}

#[derive(Clone, Debug)]
pub struct Edges {
    pub n_cities: usize,
    pub values: Vec<Vec<Edge>>,
    pub params: Parameters,
}

impl Edges {
    pub fn new(cities: &[City], params: &Parameters) -> Self {
        let params = *params;
        let n_cities = cities.len();
        assert!(cities.len() > 2);
        Self {
            n_cities,
            values: (0..n_cities)
                .map(|i| {
                    (0..n_cities)
                        .map(|j| {
                            let distance = cities[i].distance(&cities[j]);
                            Edge {
                                distance,
                                attractiveness: if i != j { 1.0 / distance } else { 0.0 },
                                total_trail: 1e-3,
                                recent_trail: 0.0,
                            }
                        })
                        .collect()
                })
                .collect(),
            params,
        }
    }

    pub fn deposit_trail(&mut self, i: usize, j: usize, trail: f32) {
        self.values[i][j].recent_trail += trail;
        self.values[j][i].recent_trail += trail;
    }

    pub fn apply_decay(&mut self) {
        let multiplier = 1.0 - self.params.trail_decay;
        for edge in self.values.iter_mut().flatten() {
            edge.total_trail = edge.total_trail * multiplier + edge.recent_trail;
            edge.recent_trail = 0.0;
        }
    }
}
