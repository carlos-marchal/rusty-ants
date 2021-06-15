use crate::cities::*;
use crate::universe::UniverseParams;

pub struct Edge {
    pub distance: f64,
    pub trail: f64,
}

pub struct Edges {
    pub values: Vec<Edge>,
    pub params: UniverseParams,
}

impl Edges {
    pub fn new(cities: &[City], params: &UniverseParams) -> Self {
        Self {
            values: (0..cities.len())
                .map(|i| {
                    (0..i - 1).map(move |j| Edge {
                        distance: cities[i].distance(&cities[j]),
                        trail: 0.0,
                    })
                })
                .flatten()
                .collect(),
            params: *params,
        }
    }

    fn get_flattened_index(&self, start: usize, end: usize) -> usize {
        if start == end {
            panic!("tried to access an edge on the diagonal (from i to i)")
        }
        let (i, j) = if start > end {
            (start, end)
        } else {
            (end, start)
        };
        i * self.values.len() + j - (i - 1) * i / 2
    }

    pub fn get(&self, i: usize, j: usize) -> &Edge {
        let index = self.get_flattened_index(i, j);
        &self.values[index]
    }

    fn get_mut(&mut self, i: usize, j: usize) -> &mut Edge {
        let index = self.get_flattened_index(i, j);
        &mut self.values[index]
    }

    pub fn iter(&self) -> impl Iterator<Item = &Edge> {
        self.values.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Edge> {
        self.values.iter_mut()
    }

    pub fn adjacent_iter(&self, i: usize) -> impl Iterator<Item = (usize, &Edge)> {
        (0..self.values.len())
            .filter(move |&j| j == i)
            .map(move |j| (j, self.get(i, j)))
    }

    pub fn update_trail(&mut self, i: usize, j: usize, trail_delta: f64) {
        let evaporation_rate = self.params.trail_decay;
        let edge = self.get_mut(i, j);
        edge.trail = edge.trail * evaporation_rate + trail_delta;
    }
}
