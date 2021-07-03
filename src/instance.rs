use crate::cities::City;
use crate::edges::Edges;

#[derive(Debug)]
pub struct Instance {
    n: usize,
    cities: Vec<City>,
    edges: Edges,
    tour: Vec<usize>,
    tour_length: f32,
    done: bool,
}

#[derive(Clone, Debug)]
pub struct Solution {
    pub tour: Vec<usize>,
    pub tour_length: f32,
}

impl Instance {
    pub fn new(cities: &[City]) -> Self {
        let n = cities.len();
        let cities = cities.to_vec();
        let edges = Edges::new(&cities);
        let tour: Vec<_> = (0..cities.len()).collect();
        let tour_length = tour
            .windows(2)
            .map(|window| match window {
                &[start, end] => edges.distances[start][end],
                _ => unreachable!(),
            })
            .sum::<f32>()
            + edges.distances[n - 1][0];
        Self {
            n,
            cities,
            edges,
            tour,
            tour_length,
            done: false,
        }
    }

    pub fn is_done(&self) -> bool {
        return self.done;
    }

    pub fn get_cities(&self) -> &[City] {
        &self.cities
    }

    pub fn get_solution(&self) -> Solution {
        Solution {
            tour: self.tour.to_owned(),
            tour_length: self.tour_length,
        }
    }

    pub fn try_improvement(&mut self) -> bool {
        if self.done {
            return false;
        }
        for i in 0..(self.n - 1) {
            for j in (i + 1)..self.n {
                // Handle the special case where we would reverse the whole
                // array.
                if i == 0 && j == self.n - 1 {
                    continue;
                }
                let outer_start = self.tour[(i + self.n - 1) % self.n];
                let inner_start = self.tour[i];
                let inner_end = self.tour[j];
                let outer_end = self.tour[(j + 1) % self.n];
                let removed_cost = self.edges.distances[outer_start][inner_start]
                    + self.edges.distances[inner_end][outer_end];
                let added_cost = self.edges.distances[outer_start][inner_end]
                    + self.edges.distances[inner_start][outer_end];
                let delta = added_cost - removed_cost;
                if delta < 0.0 {
                    self.tour[i..(j + 1)].reverse();
                    self.tour_length += delta;
                    return true;
                }
            }
        }
        self.done = true;
        return true;
    }
}
