pub struct Params {
    pub alpha: f64,
    pub gamma: f64,
    pub rho: f64,
    pub delta: f64
}

impl Params {
    pub fn default() -> Params {
        Params { alpha: 1.0, gamma: 2.0, rho: 0.5, delta: 0.5 }
    }
}
