use js_sys::Math::random;

pub struct City {
    x: f64,
    y: f64,
}

impl City {
    pub fn distance(&self, city: &Self) -> f64 {
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
