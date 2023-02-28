use std::f64::EPSILON;

mod bound_op;
mod card;
mod op;
pub use bound_op::*;
pub use card::*;
pub use op::*;

pub fn abs_diff_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
