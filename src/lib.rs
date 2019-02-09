//! A Nelder-Mead simplex optimizer

pub mod params;
pub mod bounds;
mod simplex;
mod algebra;

use crate::simplex::*;
use crate::params::*;
use crate::bounds::*;

pub fn minimize(
    f: &Function,
    initial_point: Point,
    initial_simplex_size: f64,
    params: Params,
    bounds: Bounds,
    max_iter: u32,
) -> (Point, f64) {
    let initial_simplex = new_simplex(f, initial_point, initial_simplex_size);
    crate::simplex::minimize(f, initial_simplex, params, bounds, max_iter)
}
