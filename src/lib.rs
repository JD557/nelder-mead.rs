//! A Nelder-Mead simplex optimizer

mod algebra;
pub mod bounds;
pub mod params;
mod simplex;

use crate::bounds::*;
use crate::params::*;
use crate::simplex::*;

pub fn minimize(
    f: impl Fn(&Vec<f64>) -> f64,
    initial_point: Vec<f64>,
    initial_simplex_size: f64,
    params: Params,
    bounds: Bounds,
    max_iter: u32,
) -> (Vec<f64>, f64) {
    let initial_simplex = new_simplex(&f, initial_point, initial_simplex_size);
    crate::simplex::minimize(&f, initial_simplex, params, bounds, max_iter)
}
