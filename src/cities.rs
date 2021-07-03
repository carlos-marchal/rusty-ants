#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct City {
    pub x: f32,
    pub y: f32,
}

impl City {
    pub fn distance(&self, city: &Self) -> f32 {
        let dx = self.x - city.x;
        let dy = self.y - city.y;
        (dx * dx + dy * dy).sqrt()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_calculates_distances_correctly() {
        for (a, b, result) in vec![
            ((0.0, 0.0), (0.0, 0.0), 0.0),
            ((1.0, 0.0), (0.0, 0.0), 1.0),
            ((0.0, 0.0), (0.0, 1.0), 1.0),
            ((-1.0, 0.0), (0.0, 0.0), 1.0),
            ((10.0, 0.0), (-10.0, 0.0), 20.0),
            ((1.0, 0.0), (0.0, -1.0), (1.4142135623730951)),
        ] {
            assert_eq!(
                City { x: a.0, y: a.1 }.distance(&City { x: b.0, y: b.1 }),
                result
            )
        }
    }
}
