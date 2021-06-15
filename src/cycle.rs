use crate::edges::{Edge, Edges};
use crate::universe::UniverseParams;
use js_sys::Math::random;

#[derive(Clone, Debug)]
pub struct Ant {
    pub visited: Vec<bool>,
    pub tour: Vec<usize>,
}

#[derive(Debug)]
pub struct Cycle<'a> {
    pub edges: &'a mut Edges,
    pub params: UniverseParams,
    pub time: usize,
    pub ants: Vec<Ant>,
}

#[derive(Clone, Debug)]
pub struct CycleResult {
    pub shortest_tour: Vec<usize>,
    pub shortest_length: f64,
}

impl<'a> Cycle<'a> {
    pub fn new(edges: &'a mut Edges, params: &UniverseParams) -> Self {
        let n = edges.values.len();
        Cycle {
            edges,
            params: *params,
            time: 1,
            ants: (0..n)
                .map(|i| Ant {
                    visited: (0..n).map(|j| i == j).collect(),
                    tour: vec![],
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
                .adjacent_iter(*ant.tour.last().unwrap())
                .filter(|&(i, _)| !ant.visited[i])
                .map(
                    |(
                        i,
                        Edge {
                            distance, trail, ..
                        },
                    )| {
                        (
                            i,
                            distance.powf(distance_importance) * trail.powf(trail_importance),
                        )
                    },
                )
                .collect();
            let sum: f64 = adjacents.iter().map(|(_, weight)| weight).sum();

            let target = 1.0 - random();
            let mut accumulated = 0.0;
            for (index, weight) in adjacents {
                accumulated += weight / sum;
                if accumulated >= target {
                    ant.visited[index] = true;
                    ant.tour.push(index);
                    break;
                }
            }
        }
    }

    pub fn complete(mut self) -> CycleResult {
        let n = self.edges.values.len();
        while self.time < n {
            self.tick();
        }

        let mut shortest_tour: Vec<usize> = vec![];
        let mut shortest_length = f64::INFINITY;
        for mut ant in self.ants {
            let mut tour_length = 0.0;
            ant.tour.push(ant.tour[0]);
            let tour = &ant.tour;
            for window in tour.windows(2) {
                let (start, end) = (window[0], window[1]);
                tour_length += self.edges.get(start, end).distance;
            }

            let trail_delta = self.params.distance_constant / tour_length;
            for window in tour.windows(2) {
                let (start, end) = (window[0], window[1]);
                self.edges.add_trail(start, end, trail_delta);
            }

            if tour_length < shortest_length {
                shortest_length = tour_length;
                shortest_tour = ant.tour;
            }
        }

        self.edges.apply_decay();

        CycleResult {
            shortest_tour,
            shortest_length,
        }
    }
}
