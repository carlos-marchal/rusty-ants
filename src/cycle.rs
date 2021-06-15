use crate::edges::{Edge, Edges};
use crate::universe::UniverseParams;
use js_sys::Math::random;

pub struct Ant {
    pub visited: Vec<bool>,
    pub location: usize,
}

pub struct Cycle<'a> {
    pub edges: &'a Edges,
    pub params: UniverseParams,
    pub time: usize,
    pub ants: Vec<Ant>,
}

impl<'a> Cycle<'a> {
    pub fn new(edges: &'a Edges, params: &UniverseParams) -> Self {
        Cycle {
            edges,
            params: *params,
            time: 1,
            ants: (0..edges.values.len())
                .map(|i| Ant {
                    visited: (0..edges.values.len()).map(|j| i == j).collect(),
                    location: i,
                })
                .collect(),
        }
    }

    pub fn tick(&mut self) {
        let UniverseParams {
            distance_importance,
            trail_importance,
            ..
        } = self.params;

        for ant in self.ants.iter_mut() {
            let adjacents: Vec<_> = self
                .edges
                .adjacent_iter(ant.location)
                .filter(|&(i, _)| !ant.visited[i])
                .map(|(i, Edge { distance, trail })| {
                    (
                        i,
                        distance.powf(distance_importance) * trail.powf(trail_importance),
                    )
                })
                .collect();
            let sum: f64 = adjacents.iter().map(|(_, weight)| weight).sum();

            let target = 1.0 - random();
            let mut accumulated = 0.0;
            for (index, weight) in adjacents {
                accumulated += weight / sum;
                if accumulated >= target {
                    ant.visited[index] = true;
                    ant.location = index;
                    break;
                }
            }
        }
    }
}
