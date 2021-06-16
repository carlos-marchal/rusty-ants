use crate::edges::Edges;
use crate::universe::UniverseParams;
use rand::random;

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
        let n = edges.n_cities;
        Cycle {
            edges,
            params: *params,
            time: 1,
            ants: (0..n)
                .map(|i| Ant {
                    visited: (0..n).map(|j| i == j).collect(),
                    tour: vec![i],
                })
                .collect(),
        }
    }

    pub fn tick(&mut self) -> bool {
        if self.time >= self.edges.n_cities {
            return false;
        }
        let UniverseParams {
            distance_importance,
            trail_importance,
            ..
        } = self.params;

        for ant in self.ants.iter_mut() {
            let adjacent_probability: Vec<_> = self
                .edges
                .adjacent(ant.tour[self.time - 1])
                .filter(|&(i, _)| !ant.visited[i])
                .map(|(i, edge)| {
                    (
                        i,
                        (1.0 / edge.distance).powf(distance_importance)
                            * edge.trail.powf(trail_importance),
                    )
                })
                .collect();
            let sum: f64 = adjacent_probability.iter().map(|(_, weight)| weight).sum();

            let mut target: f64 = random();
            target = 1.0 - target;
            let mut accumulated = 0.0;
            for (index, weight) in adjacent_probability {
                accumulated += weight / sum;
                if accumulated >= target {
                    ant.visited[index] = true;
                    ant.tour.push(index);
                    break;
                }
            }
        }
        self.time += 1;
        true
    }

    pub fn complete(mut self) -> CycleResult {
        // Execute all ticks in the cycle
        while self.tick() {}

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

        let result = CycleResult {
            shortest_tour,
            shortest_length,
        };
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cities::City;
    use crate::edges::Edges;

    fn get_test_data() -> (Vec<City>, Edges) {
        let cities = vec![
            City { x: 0.0, y: 0.0 },
            City { x: 1.0, y: 0.0 },
            City { x: 2.0, y: 0.0 },
            City { x: 3.0, y: 0.0 },
            City { x: 4.0, y: 0.0 },
            City { x: 5.0, y: 0.0 },
            City { x: 0.0, y: 3.0 },
        ];
        let edges = Edges::new(&cities, &Default::default());
        (cities, edges)
    }

    #[test]
    fn it_places_one_ant_per_city() {
        let (cities, mut edges) = get_test_data();
        let cycle = Cycle::new(&mut edges, &Default::default());
        assert_eq!(cycle.ants.len(), cities.len());
        let mut seen = vec![false; cities.len()];
        for ant in cycle.ants {
            assert_eq!(ant.tour.len(), 1);
            seen[ant.tour[0]] = true;
        }
        assert!(seen.iter().all(|&seen| seen));
    }

    #[test]
    fn it_applies_time_correctly() {
        let (cities, mut edges) = get_test_data();
        let mut cycle = Cycle::new(&mut edges, &Default::default());
        cycle.tick();
    }
}
