use crate::cities::City;
use crate::edges::Edges;
use crate::parameters::Parameters;
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
    pub params: Parameters,
}

#[derive(Clone, Debug)]
pub struct CycleResult {
    pub shortest_tour: Vec<usize>,
    pub shortest_length: f32,
}

impl Instance {
    pub fn new(cities: &[City], params: &Parameters) -> Self {
        Self {
            n: cities.len(),
            cities: cities.to_vec(),
            edges: Edges::new(&cities, params),
            params: *params,
            cycle_count: 0,
        }
    }

    pub fn cycle(&mut self) -> CycleResult {
        let mut ants = Vec::with_capacity(self.n);
        for i in 0..self.n {
            let mut visited = vec![false; self.n];
            visited[i] = true;
            let mut tour = Vec::with_capacity(self.n + 1);
            tour.push(i);
            ants.push(Ant { visited, tour });
        }

        for ant in &mut ants {
            for i in 0..self.n - 1 {
                let city = ant.tour[i];
                let mut desirabilities = Vec::with_capacity(self.n);
                for j in 0..self.n {
                    if ant.visited[j] {
                        desirabilities.push(0.0);
                    } else {
                        let edge = self.edges.values[city][j];
                        let trail_factor = edge.total_trail.powf(self.params.trail_importance);
                        let len_factor = edge.attractiveness.powf(self.params.distance_importance);
                        desirabilities.push(trail_factor * len_factor)
                    }
                }
                let sum: f32 = desirabilities.iter().sum();
                assert_ne!(sum, 0.0);
                let mut probabilites = desirabilities;
                for probability in &mut probabilites {
                    *probability /= sum;
                }
                let target: f32 = random();
                let mut accumulated = 0.0;
                for j in 0..self.n {
                    accumulated += probabilites[j];
                    if target <= accumulated {
                        ant.tour.push(j);
                        ant.visited[j] = true;
                        break;
                    }
                }
            }
            ant.tour.push(ant.tour[0]);
            assert_eq!(51, ant.tour.len());
        }

        let mut costs = Vec::with_capacity(ants.len());
        for ant in &ants {
            let mut cost = 0.0;
            for i in 0..self.n {
                cost += self.edges.values[ant.tour[i]][ant.tour[i + 1]].distance;
            }
            costs.push(cost);
            let delta = self.params.distance_constant / cost;
            for i in 0..self.n {
                self.edges
                    .deposit_trail(ant.tour[i], ant.tour[i + 1], delta);
            }
        }
        self.edges.apply_decay();

        let mut min_index = 0;
        let mut min_cost = f32::INFINITY;
        for (index, &cost) in costs.iter().enumerate() {
            if cost < min_cost {
                min_index = index;
                min_cost = cost;
            }
        }
        CycleResult {
            shortest_length: min_cost,
            shortest_tour: ants[min_index].tour.to_owned(),
        }
    }
}
