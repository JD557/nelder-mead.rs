//! A [Nelder-Mead method][neldermead] implementation.
//!
//! Allows fast minimization/maximization of `Vec<f64> -> f64` functions.
//!
//! # Basic usage
//!
//! ```
//! use nelder_mead::*;
//! use nelder_mead::params::*;
//!
//! use assert_approx_eq::assert_approx_eq;
//!
//! // minimize (x+1)^2 + y^2
//! let (x, fx) = minimize_unbounded(
//!    |args| (args[0]+1.0) * (args[0]+1.0) + args[1]*args[1],
//!    vec![5.0,5.0],
//!    1.0,
//!    Params::default(),
//!    1000);
//!
//! // expected minimum: f(-1, 0) = 0
//! assert_approx_eq!(x[0], -1.0);
//! assert_approx_eq!(x[1], 0.0);
//! assert_approx_eq!(fx, 0.0);
//! ```
//!
//! [neldermead]: https://en.wikipedia.org/wiki/Nelder%E2%80%93Mead_method

mod algebra;
pub mod bounds;
pub mod params;
mod simplex;

use crate::bounds::*;
use crate::params::*;
use crate::simplex::*;

/// Minimizes a function `f`,
/// starting with a simplex of size `initial_simplex_size` centered on
/// `initial_point`.
///
/// The search space is bounded by a `Bounds` definition.
///
/// # Example
///
/// ```
/// use nelder_mead::*;
/// use nelder_mead::bounds::*;
/// use nelder_mead::params::*;
///
/// use assert_approx_eq::assert_approx_eq;
///
/// // minimize (x+1)^2 + y^2
/// let (x, fx) = minimize(
///    |args| (args[0]+1.0) * (args[0]+1.0) + args[1]*args[1],
///    vec![5.0,5.0],
///    1.0,
///    Params::default(),
///    Bounds {min: vec![0.0, 0.0], max: vec![10.0, 10.0]},
///    1000);
///
/// // expected bounded minimum: f(0, 0) = 1
/// assert_approx_eq!(x[0], 0.0);
/// assert_approx_eq!(x[1], 0.0);
/// assert_approx_eq!(fx, 1.0);
/// ```
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

/// Maximizes a function `f`,
/// starting with a simplex of size `initial_simplex_size` centered on
/// `initial_point`.
///
/// The search space is bounded by a `Bounds` definition.
///
/// # Example
///
/// ```
/// use nelder_mead::*;
/// use nelder_mead::bounds::*;
/// use nelder_mead::params::*;
///
/// use assert_approx_eq::assert_approx_eq;
///
/// // maximize -2.0 * ((x+1)^2 + y^2)
/// let (x, fx) = maximize(
///    |args| -2.0 * ((args[0]+1.0) * (args[0]+1.0) + args[1]*args[1]),
///    vec![5.0,5.0],
///    1.0,
///    Params::default(),
///    Bounds {min: vec![0.0, 0.0], max: vec![10.0, 10.0]},
///    1000);
///
/// // expected bounded maximum: f(0, 0) = -2
/// assert_approx_eq!(x[0], 0.0);
/// assert_approx_eq!(x[1], 0.0);
/// assert_approx_eq!(fx, -2.0);
/// ```
pub fn maximize(
    f: impl Fn(&Vec<f64>) -> f64,
    initial_point: Vec<f64>,
    initial_simplex_size: f64,
    params: Params,
    bounds: Bounds,
    max_iter: u32,
) -> (Vec<f64>, f64) {
    let g: &(Fn(&Vec<f64>) -> f64) = &(|x| -1.0 * f(x));
    let initial_simplex = new_simplex(&g, initial_point, initial_simplex_size);
    let (x, gx) = crate::simplex::minimize(&g, initial_simplex, params, bounds, max_iter);
    (x, -1.0 * gx)
}

/// Minimizes a function `f`,
/// starting with a simplex of size `initial_simplex_size` centered on
/// `initial_point`.
///
/// # Example
///
/// ```
/// use nelder_mead::*;
/// use nelder_mead::bounds::*;
/// use nelder_mead::params::*;
///
/// use assert_approx_eq::assert_approx_eq;
///
/// // minimize (x+1)^2 + y^2
/// let (x, fx) = minimize_unbounded(
///    |args| (args[0]+1.0) * (args[0]+1.0) + args[1]*args[1],
///    vec![5.0,5.0],
///    1.0,
///    Params::default(),
///    1000);
///
/// // expected bounded minimum: f(-1, 0) = 0
/// assert_approx_eq!(x[0], -1.0);
/// assert_approx_eq!(x[1], 0.0);
/// assert_approx_eq!(fx, 0.0);
/// ```
pub fn minimize_unbounded(
    f: impl Fn(&Vec<f64>) -> f64,
    initial_point: Vec<f64>,
    initial_simplex_size: f64,
    params: Params,
    max_iter: u32,
) -> (Vec<f64>, f64) {
    let bounds = Bounds::none(initial_point.len());
    minimize(f, initial_point, initial_simplex_size, params, bounds, max_iter)
}

/// Maximizes a function `f`,
/// starting with a simplex of size `initial_simplex_size` centered on
/// `initial_point`.
///
/// # Example
///
/// ```
/// use nelder_mead::*;
/// use nelder_mead::bounds::*;
/// use nelder_mead::params::*;
///
/// use assert_approx_eq::assert_approx_eq;
///
/// // maximize -2.0 * ((x+1)^2 + y^2)
/// let (x, fx) = maximize_unbounded(
///    |args| -2.0 * ((args[0]+1.0) * (args[0]+1.0) + args[1]*args[1]),
///    vec![5.0,5.0],
///    1.0,
///    Params::default(),
///    1000);
///
/// // expected bounded maximum: f(-1, 0) = 0
/// assert_approx_eq!(x[0], -1.0);
/// assert_approx_eq!(x[1], 0.0);
/// assert_approx_eq!(fx, 0.0);
/// ```
pub fn maximize_unbounded(
    f: impl Fn(&Vec<f64>) -> f64,
    initial_point: Vec<f64>,
    initial_simplex_size: f64,
    params: Params,
    max_iter: u32,
) -> (Vec<f64>, f64) {
    let bounds = Bounds::none(initial_point.len());
    maximize(f, initial_point, initial_simplex_size, params, bounds, max_iter)
}
