use lpp::Lpp;

pub use interfaces::{Constraints, Sign};
use interfaces::{InteriorPoint, NotApplicableError};

mod algorithm;
mod interfaces;
mod lpp;

pub fn interior_point(
    objective_function: Vec<f64>,
    constraints: &Constraints,
    initial_point: Vec<f64>,
    eps: usize,
    alpha: f64,
) -> Result<(Lpp, InteriorPoint), NotApplicableError> {
    let lpp = Lpp::try_new(objective_function, constraints, initial_point, eps)?;

    Ok((
        lpp.clone(),
        InteriorPoint {
            done: false,
            x: lpp.x,
            big_a: lpp.big_a,
            c: lpp.c,
            eps: lpp.eps,
            alpha,
        },
    ))
}
