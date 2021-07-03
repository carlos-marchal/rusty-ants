use crate::cities::City;
use crate::edges::Edges;

#[derive(Debug)]
pub struct Instance {
    pub n: usize,
    pub cities: Vec<City>,
    pub edges: Edges,
    pub tour: Vec<usize>,
    pub cycle_count: usize,
}

#[derive(Clone, Debug)]
pub struct CycleResult {
    pub shortest_tour: Vec<usize>,
    pub done: bool,
}

type Combination = (usize, usize, usize);

impl Instance {
    pub fn new(cities: &[City]) -> Self {
        Self {
            n: cities.len(),
            cities: cities.to_vec(),
            edges: Edges::new(&cities),
            tour: (0..cities.len()).collect(),
            cycle_count: 0,
        }
    }

    pub fn combinations(&self) -> impl Iterator<Item = Combination> {
        let n = self.n;
        (0..n)
            .map(move |i| {
                (i + 2..n).map(move |j| {
                    let end = if i > 0 { n + 1 } else { n };
                    (j + 2..end).map(move |k| (i, j, k))
                })
            })
            .flatten()
            .flatten()
    }

    pub fn try_combination(&mut self, combination: &Combination) -> bool {
        let &(i, j, k) = combination;
        let a = self.tour[if i == 0 { self.tour.len() - 1 } else { i }];
        let b = self.tour[i];
        let c = self.tour[j - 1];
        let d = self.tour[j];
        let e = self.tour[k - 1];
        let f = self.tour[k % self.tour.len()];

        let distances = &self.edges.distances;
        let current = distances[a][b] + distances[c][d] + distances[e][f];

        let first_try = distances[a][c] + distances[b][d] + distances[e][f];
        if first_try < current {
            println!("first");
            self.tour[i..j].reverse();
            return true;
        }

        let second_try = distances[a][b] + distances[c][e] + distances[d][f];
        if second_try < current {
            println!("second");
            self.tour[j..k].reverse();
            return true;
        }

        let third_try = distances[a][d] + distances[e][b] + distances[c][f];
        if third_try < current {
            println!("third");
            self.tour[i..k].reverse();
            return true;
        }

        let fourth_try = distances[f][b] + distances[c][d] + distances[e][a];
        if fourth_try < current {
            println!("fourth");
            let reversal: Vec<usize> = self.tour[j..k]
                .iter()
                .chain(self.tour[i..j].iter()).cloned()
                .collect();
            self.tour.splice(i..k, reversal);
            return true;
        }

        return false;
    }

    pub fn cycle(&mut self) -> CycleResult {
        for combination in self.combinations() {
            let improved = self.try_combination(&combination);
            println!("improved");
            if improved {
                return CycleResult {
                    shortest_tour: self.tour.to_vec(),
                    done: false,
                };
            }
        }
        CycleResult {
            shortest_tour: self.tour.to_vec(),
            done: true,
        }
    }
}