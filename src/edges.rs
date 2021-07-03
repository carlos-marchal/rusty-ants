use crate::cities::*;

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
    pub distances: Vec<Vec<f32>>,
}

impl Edges {
    pub fn new(cities: &[City]) -> Self {
        let n_cities = cities.len();
        assert!(cities.len() > 2);
        Self {
            n_cities,
            distances: (0..n_cities)
                .map(|i| {
                    (0..n_cities)
                        .map(|j| cities[i].distance(&cities[j]))
                        .collect()
                })
                .collect(),
        }
    }
}
