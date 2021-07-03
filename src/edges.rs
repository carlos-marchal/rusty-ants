use crate::cities::*;

#[derive(Clone, Debug)]
pub struct Edges {
    pub n_cities: usize,
    pub distances: Vec<Vec<f32>>,
}

impl Edges {
    pub fn new(cities: &[City]) -> Self {
        let n_cities = cities.len();
        assert!(cities.len() > 2);
        Self {
            n_cities,
            distances: (0..n_cities)
                .map(|i| {
                    (0..n_cities)
                        .map(|j| cities[i].distance(&cities[j]))
                        .collect()
                })
                .collect(),
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
    fn it_has_correct_shape() {
        let cities = get_test_cities();
        for multiple in 1..10 {
            let size = multiple * cities.len();
            let cities: Vec<_> = cities.iter().cycle().take(size).copied().collect();
            let edges = Edges::new(&cities);
            assert_eq!(edges.distances.len(), size);
            for row in edges.distances {
                assert_eq!(row.len(), size)
            }
        }
    }

    #[test]
    fn it_is_symmetric_matrix() {
        let cities = get_test_cities();
        let edges = Edges::new(&cities);
        for i in 0..edges.n_cities {
            for j in 0..edges.n_cities {
                assert_eq!(edges.distances[i][j], edges.distances[j][i]);
            }
        }
        for i in 0..edges.n_cities {
            assert_eq!(edges.distances[i][i], 0.0);
        }
    }
}
