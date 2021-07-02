use rand::random;

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

pub fn generate(n: usize) -> Vec<City> {
    (0..n)
        .map(|_| City {
            x: random(),
            y: random(),
        })
        .collect()
}