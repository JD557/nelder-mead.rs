use crate::simplex::Point;

pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Bounds {
    pub fn none(n: usize) -> Bounds {
        let mut min = Vec::new();
        let mut max = Vec::new();
        for _ in 0..n {
            min.push(std::f64::MIN);
            max.push(std::f64::MAX);
        }
        Bounds { min, max }
    }

    pub fn as_vec(self: &Bounds) -> Vec<(f64, f64)> {
        self.min
            .iter()
            .cloned()
            .zip(self.max.iter().cloned())
            .collect()
    }
}
