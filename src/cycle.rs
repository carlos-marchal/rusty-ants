use crate::cities::City;
use crate::edges::{Edge, Edges};
use crate::universe::UniverseParams;
use js_sys::Math::random;

pub struct Ant {
    visited: Vec<bool>,
    location: usize,
}

pub struct Cycle<'a> {
    cities: &'a [City],
    edges: &'a Edges<'a>,
    params: &'a UniverseParams,
    time: usize,
    ants: Vec<Ant>,
}

impl<'a> Cycle<'a> {
    pub fn new(cities: &'a [City], edges: &'a Edges, params: &'a UniverseParams) -> Cycle<'a> {
        let mut cycle = Cycle {
            cities,
            edges,
            params,
            time: 0,
            ants: (0..cities.len())
                .map(|_| Ant {
                    visited: vec![false; cities.len()],
                    location: 0,
                })
                .collect(),
        };
        cycle.restart();
        cycle
    }

    pub fn restart(&mut self) {
        self.time = 1;
        for (i, ant) in self.ants.iter_mut().enumerate() {
            ant.location = i;
            for (j, visited) in ant.visited.iter_mut().enumerate() {
                *visited = i == j
            }
        }
    }

    pub fn tick(&mut self) {
        let &UniverseParams {
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
