extern crate rand;
use rand::rngs::OsRng;
use rand::Rng;

use crate::algebra::*;
use crate::bounds::*;
use crate::params::*;

pub type Point = Vec<f64>;
pub type Function = (Fn(&Point) -> f64);
type Simplex = Vec<(Point, f64)>;

fn sort_simplex(simplex: &mut Simplex) {
    simplex.sort_by(|(_, fx), (_, fy)| fx.partial_cmp(fy).unwrap());
}

fn add_point(f: &Function, simplex: Simplex, point: Vec<f64>) -> Simplex {
    let mut new_simplex = simplex.clone();
    new_simplex.push((point.clone(), f(&point)));
    sort_simplex(&mut new_simplex);
    new_simplex.truncate(new_simplex.len() - 1);
    new_simplex
}

fn step(f: &Function, simplex: Simplex, params: &Params, bounds_vec: &Vec<(f64, f64)>) -> Simplex {
    let n = simplex.len() - 1;
    let x1 = simplex[0].0.clone();
    let fx1 = simplex[0].1;
    let x0 = {
        let median_list = &simplex.as_slice()[0..n];
        avg(&median_list
            .iter()
            .map(|x| x.0.clone())
            .collect::<Vec<Point>>())
    };
    let (_xn, fxn) = simplex[n - 1].clone();
    let (xn1, fxn1) = simplex[n].clone();

    let xr = clamp(
        &sum(&x0, &mult(params.alpha, &diff(&x0, &xn1))),
        &bounds_vec,
    );
    let fxr = f(&xr);
    let xe = clamp(&sum(&x0, &mult(params.gamma, &diff(&xr, &x0))), &bounds_vec);
    let fxe = f(&xe);
    let xc = clamp(&sum(&x0, &mult(params.rho, &diff(&xn1, &x0))), &bounds_vec);
    let fxc = f(&xc);

    if fx1 <= fxr && fxr < fxn {
        // Reflection
        add_point(f, simplex, xr)
    } else if fxe < fxn1 {
        // Expansion
        if fxe < fxr {
            add_point(f, simplex, xe)
        } else {
            add_point(f, simplex, xr)
        }
    } else if fxc < fxn1 {
        // Contraction
        add_point(f, simplex, xc)
    } else {
        // Shrink
        let mut new_points: Vec<(Vec<f64>, f64)> = simplex
            .iter()
            .skip(1)
            .map(|(xi, _)| sum(&x1, &mult(params.delta, &diff(&xi, &x1))))
            .map(|xi| (xi.clone(), f(&xi)))
            .collect();
        new_points.push((x1, fx1));
        sort_simplex(&mut new_points);
        new_points
    }
}

pub fn minimize(
    f: &Function,
    initial_simplex: Simplex,
    params: Params,
    bounds: Bounds,
    max_iter: u32,
) -> (Point, f64) {
    let bounds_vec = bounds.as_vec();
    let mut curr_simplex = initial_simplex.clone();
    let n = curr_simplex.len() - 1;
    for _ in 0..max_iter {
        curr_simplex = step(f, curr_simplex, &params, &bounds_vec);
    }
    let x1 = curr_simplex[0].0.clone();
    let fx1 = curr_simplex[0].1;
    let x0 = {
        let median_list = &curr_simplex.as_slice()[0..n];
        avg(&median_list
            .iter()
            .map(|x| x.0.clone())
            .collect::<Vec<Point>>())
    };
    let fx0 = f(&x0);
    if fx1 < fx0 {
        (x1, fx1)
    } else {
        (x0, fx0)
    }
}

pub fn new_simplex(f: &Function, center: Point, step: f64) -> Simplex {
    let mut rng = OsRng::new().expect("Failed to create the RNG");
    let mut unsorted_points: Vec<Point> = Vec::new();
    for _ in 0..center.len() + 1 {
        let new_point = center
            .iter()
            .map(|x| x + rng.gen_range(-step, step))
            .collect();
        unsorted_points.push(new_point);
    }
    let mut sorted_points: Vec<(Point, f64)> =
        unsorted_points.iter().map(|x| (x.clone(), f(x))).collect();
    sort_simplex(&mut sorted_points);
    sorted_points
}

#[cfg(test)]
mod tests {
    extern crate assert_approx_eq;
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn minimize_square() {
        let f: &Function = &(|args| args[0] * args[0] + args[1] * args[1] + 5.0);
        let initial_simplex = new_simplex(&f, vec![2.0, 2.0], 0.5);
        let (point, value) = minimize(f, initial_simplex, Params::default(), Bounds::none(2), 500);
        assert_approx_eq!(point[0], 0.0);
        assert_approx_eq!(point[1], 0.0);
        assert_approx_eq!(value, 5.0);
    }

    #[test]
    fn minimize_with_bounds() {
        let f: &Function = &(|args| args[0] + args[1] + 5.0);
        let bounds = Bounds {
            min: vec![-1.0, 0.5],
            max: vec![10.0, 10.0],
        };
        let initial_simplex = new_simplex(&f, vec![2.0, 2.0], 0.5);
        let (point, value) = minimize(f, initial_simplex, Params::default(), bounds, 500);
        assert_approx_eq!(point[0], -1.0);
        assert_approx_eq!(point[1], 0.5);
        assert_approx_eq!(value, 4.5);
    }
}
