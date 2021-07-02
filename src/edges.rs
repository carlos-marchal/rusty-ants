use crate::cities::*;
use crate::universe::Parameters;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    pub distance: f32,
    pub trail: f32,
    pub trail_delta: f32,
}

#[derive(Clone, Debug)]
pub struct Edges {
    pub n_cities: usize,
    pub values: Vec<Edge>,
    pub params: Parameters,
}

impl Edges {
    pub fn new(cities: &[City], params: &Parameters) -> Self {
        let params = *params;
        let n_cities = cities.len();
        assert!(cities.len() > 2);
        Self {
            n_cities,
            values: (1..n_cities)
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

    fn get_flattened_index(&self, start: usize, end: usize) -> usize {
        assert_ne!(start, end);
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

    pub fn adjacent(&self, i: usize) -> impl Iterator<Item = (usize, &Edge)> {
        (0..i)
            .chain(i + 1..self.n_cities)
            .map(move |j| (j, self.get(i, j)))
    }

    pub fn add_trail(&mut self, i: usize, j: usize, trail_delta: f32) {
        let edge = self.get_mut(i, j);
        edge.trail_delta += trail_delta;
    }

    pub fn apply_decay(&mut self) {
        let multiplier = 1.0 - self.params.trail_decay;
        for edge in &mut self.values {
            edge.trail = edge.trail * multiplier + edge.trail_delta;
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
    fn it_stores_values_in_flattened_lower_triangular_matrix() {
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
    fn it_gets_correct_edge() {
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
    fn it_returns_adjacent_edge_iterator() {
        let cities = get_test_cities();
        let edges = Edges::new(&cities, &Default::default());
        for i in 0..cities.len() {
            let adjacent = (0..i).chain(i + 1..cities.len()).map(|j| edges.get(i, j));
            assert!(edges.adjacent(i).map(|(_, edge)| edge).eq(adjacent));
        }
    }

    #[test]
    fn it_accumulates_trail() {
        let cities = get_test_cities();
        let mut edges = Edges::new(&cities, &Default::default());
        let mut old_edges = edges.clone();
        for (i, j, delta) in vec![(0, 4, 10.0), (1, 0, 0.0), (1, 0, 100.0), (1, 3, -1.0)] {
            edges.add_trail(i, j, delta);
            let index = edges.get_flattened_index(i, j);
            for (i, (new, old)) in edges.values.iter().zip(old_edges.values.iter()).enumerate() {
                if i == index {
                    assert_eq!(old.trail_delta + delta, new.trail_delta);
                } else {
                    assert_eq!(old.trail_delta, new.trail_delta);
                }
                assert_eq!(old.distance, new.distance);
                assert_eq!(old.trail, new.trail);
            }
            old_edges = edges.clone();
        }
    }

    #[test]
    fn it_applies_trail_decay() {
        let cities = vec![City { x: 0.0, y: 0.0 }, City { x: 1.0, y: 1.0 }];
        let params: Parameters = Default::default();
        let mut edges = Edges::new(&cities, &params);
        let trail = 10.0;
        let delta = 4.0;
        edges.values[0].trail = trail;
        edges.add_trail(0, 1, delta);
        edges.apply_decay();
        let expected = trail * params.trail_decay + delta;
        assert_eq!(edges.values[0].trail, expected);
        assert_eq!(edges.values[0].trail_delta, 0.0);
    }
}
