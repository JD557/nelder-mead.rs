# Nelder-Mead.rs

A [Nelder-Mead method][neldermead] implementation in Rust.

Allows fast minimization/maximization of `Vec<f64> -> f64` functions.

## Basic usage

```
use nelder_mead::*;
use nelder_mead::params::*;

use assert_approx_eq::assert_approx_eq;

// minimize (x+1)^2 + y^2
let (x, fx) = minimize_unbounded(
   |args| (args[0]+1.0) * (args[0]+1.0) + args[1]*args[1],
   vec![5.0,5.0],
   1.0,
   Params::default(),
   1000);

// expected minimum: f(-1, 0) = 0
assert_approx_eq!(x[0], -1.0);
assert_approx_eq!(x[1], 0.0);
assert_approx_eq!(fx, 0.0);
```

[neldermead]: https://en.wikipedia.org/wiki/Nelder%E2%80%93Mead_method
