use crate::cities::*;
use crate::universe::UniverseParams;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    pub distance: f64,
    pub trail: f64,
    pub trail_delta: f64,
}

#[derive(Clone, Debug)]
pub struct Edges {
    pub n: usize,
    pub values: Vec<Edge>,
    pub params: UniverseParams,
}

impl Edges {
    pub fn new(cities: &[City], params: &UniverseParams) -> Self {
        let params = *params;
        let n = cities.len();
        if n < 2 {
            Self {
                n,
                values: vec![],
                params,
            }
        } else {
            Self {
                n,
                values: (1..n)
                    .map(|i| {
                        (0..i).map(move |j| Edge {
                            distance: cities[i].distance(&cities[j]),
                            trail: 0.0,
                            trail_delta: 0.0,
                        })
                    })
                    .flatten()
                    .collect(),
                params,
            }
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
        i * (i - 1) / 2 + j
    }

    pub fn get(&self, i: usize, j: usize) -> &Edge {
        &self.values[self.get_flattened_index(i, j)]
    }

    fn get_mut(&mut self, i: usize, j: usize) -> &mut Edge {
        let index = self.get_flattened_index(i, j);
        &mut self.values[index]
    }

    pub fn adjacent_iter(&self, i: usize) -> impl Iterator<Item = (usize, &Edge)> {
        (0..i)
            .chain(i + 1..self.n)
            .map(move |j| (j, self.get(i, j)))
    }

    pub fn add_trail(&mut self, i: usize, j: usize, trail_delta: f64) {
        let edge = self.get_mut(i, j);
        edge.trail_delta += trail_delta;
    }

    pub fn apply_decay(&mut self) {
        for edge in &mut self.values {
            edge.trail += edge.trail * self.params.trail_decay + edge.trail_delta;
            edge.trail_delta = 0.0;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_test_cities() -> Vec<City> {
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
    fn values_is_flattened_lower_triangular_matrix() {
        let sizes: Vec<isize> = vec![0, 1, 2, 3, 4, 5, 20, 50, 1000];
        for n in sizes {
            let edges = Edges::new(
                &vec![City { x: 0.0, y: 0.0 }; n as usize],
                &Default::default(),
            );
            let size: isize = std::cmp::max(n * (n - 1) / 2, 0);
            assert_eq!(edges.values.len(), size as usize);
        }
    }

    #[test]
    fn get_returns_correct_edge() {
        let cities = get_test_cities();
        let edges = Edges::new(&cities, &Default::default());
        for (i, j, result) in vec![
            (0, 1, 1.0),
            (1, 0, 1.0),
            (0, 5, 5.0),
            (5, 0, 5.0),
            (0, 2, 2.0),
            (1, 4, 3.0),
            (0, 6, 3.0),
            (4, 6, 5.0),
        ] {
            assert_eq!(edges.get(i, j).distance, result)
        }
    }

    #[test]
    fn adjacent_iter_returns_correct_edges() {
        let cities = get_test_cities();
        let edges = Edges::new(&cities, &Default::default());
        for i in 0..cities.len() {
            let adjacent = (0..i).chain(i + 1..cities.len()).map(|j| edges.get(i, j));
            assert!(edges.adjacent_iter(i).map(|(_, edge)| edge).eq(adjacent));
        }
    }
}
