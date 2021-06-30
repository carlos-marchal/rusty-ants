use crate::cities::City;
use crate::edges::Edges;
use crate::universe::UniverseParams;
use rand::random;

#[derive(Clone, Debug)]
pub struct Ant {
    pub visited: Vec<bool>,
    pub tour: Vec<usize>,
}

#[derive(Debug)]
pub struct Instance {
    pub n: usize,
    pub cities: Vec<City>,
    pub edges: Edges,
    pub cycle_count: usize,
    pub params: UniverseParams,
    pub time: usize,
    pub ants: Vec<Ant>,
}

#[derive(Clone, Debug)]
pub struct CycleResult {
    pub shortest_tour: Vec<usize>,
    pub shortest_length: f32,
}

fn fast_pow(base: f32, exp: f32) -> f32 {
    if exp % 1.0 < 1e-5 {
        let mut exp = exp as usize;
        if exp == 1 {
            base
        } else {
            let mut result = base;
            if exp % 2 != 0 {
                result *= base;
                exp -= 1;
            }
            while exp > 1 {
                result *= result;
                exp >>= 1;
            }
            result
        }
    } else {
        f32::powf(base, exp)
    }
}

impl Instance {
    pub fn new(cities: &[City], params: &UniverseParams) -> Self {
        let n = cities.len();
        let mut cycle = Self {
            n,
            cities: cities.to_vec(),
            edges: Edges::new(&cities, params),
            params: *params,
            time: 1,
            cycle_count: 0,
            ants: vec![
                Ant {
                    tour: Vec::with_capacity(n + 1),
                    visited: vec![false; n]
                };
                n
            ],
        };
        cycle.reset_cycle();
        cycle
    }

    fn reset_cycle(&mut self) {
        for (i, ant) in self.ants.iter_mut().enumerate() {
            ant.tour.clear();
            ant.tour.push(i);
            for (j, city) in ant.visited.iter_mut().enumerate() {
                *city = i == j;
            }
        }
        self.time = 1;
    }

    fn tick(&mut self) -> bool {
        if self.time >= self.n {
            return false;
        }
        let UniverseParams {
            distance_importance,
            trail_importance,
            ..
        } = self.params;

        for ant in self.ants.iter_mut() {
            let mut adjacent_probability: Vec<(usize, f32)> = Vec::with_capacity(self.n - 1);
            for (city, edge) in self.edges.adjacent(ant.tour[self.time - 1]) {
                if !ant.visited[city] {
                    adjacent_probability.push((
                        city,
                        fast_pow(1.0 / edge.distance, distance_importance)
                            * fast_pow(edge.trail, trail_importance)
                            + 1.0,
                    ))
                }
            }
            let sum: f32 = adjacent_probability.iter().map(|(_, weight)| weight).sum();
            let mut cumulative_probability: Vec<(usize, f32)> =
                Vec::with_capacity(adjacent_probability.len());
            for (city, probability) in adjacent_probability {
                let accumulated = cumulative_probability
                    .last()
                    .map(|&(_, value)| value)
                    .unwrap_or(0.0);
                cumulative_probability.push((city, accumulated + probability / sum));
            }
            let target: f32 = random();
            let mut assigned = false;
            for (city, accumulated) in cumulative_probability {
                if accumulated > target {
                    ant.visited[city] = true;
                    ant.tour.push(city);
                    assigned = true;
                    break;
                }
            }
            assert!(assigned);
        }
        self.time += 1;
        true
    }

    pub fn cycle(&mut self) -> CycleResult {
        // Execute all ticks in the cycle
        while self.tick() {}

        let mut shortest_tour: Option<&[usize]> = None;
        let mut shortest_length = f32::INFINITY;
        for ant in &mut self.ants {
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
                shortest_tour = Some(tour);
            }
        }

        let result = CycleResult {
            shortest_tour: shortest_tour.unwrap().to_vec(),
            shortest_length,
        };

        self.edges.apply_decay();
        self.cycle_count += 1;
        self.reset_cycle();

        result
    }

    pub fn solve(mut self) -> CycleResult {
        let mut best_result: Option<CycleResult> = None;
        while self.cycle_count < self.params.max_cycles {
            let result = self.cycle();
            match &best_result {
                None => best_result = Some(result),
                Some(solution) => {
                    if solution.shortest_length > result.shortest_length {
                        best_result = Some(result)
                    }
                }
            }
        }
        best_result.unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cities::City;

    fn get_test_data() -> Vec<City> {
        vec![
            City { x: 0.0, y: 0.0 },
            City { x: 1.0, y: 0.0 },
            City { x: 2.0, y: 0.0 },
            City { x: 3.0, y: 0.0 },
            City { x: 4.0, y: 0.0 },
            City { x: 5.0, y: 0.0 },
            City { x: 0.0, y: 3.0 },
        ]
    }

    #[test]
    fn it_places_one_ant_per_city() {
        let cities = get_test_data();
        let cycle = Instance::new(&cities, &Default::default());
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
        let cities = get_test_data();
        let mut cycle = Instance::new(&cities, &Default::default());
        cycle.tick();
    }
}
