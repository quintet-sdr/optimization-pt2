use na::{DMatrix, DVector};

pub use crate::interfaces::Sign;
use crate::interfaces::{InteriorPoint, NotApplicableError};

mod algorithm;
mod interfaces;

#[allow(clippy::many_single_char_names)]
pub fn interior_point(
    objective_function: Vec<f64>,
    constraints: &[(&[f64], Sign, f64)],
    initial_point: Vec<f64>,
    eps: usize,
    alpha: f64,
) -> Result<InteriorPoint, NotApplicableError> {
    let n = constraints.len();
    let m = constraints.first().ok_or(NotApplicableError)?.0.len();

    if initial_point.len() != n + m
        || constraints
            .iter()
            .any(|row| row.0.len() != objective_function.len())
    {
        return Err(NotApplicableError);
    }

    let initial_point_is_feasible = constraints.iter().all(|(coefficients, sign, rhs)| {
        let constraint_sum: f64 = coefficients
            .iter()
            .zip(&initial_point)
            .map(|(coeff, x)| coeff * x)
            .sum();

        sign.compare(&constraint_sum, rhs)
    });

    if !initial_point_is_feasible {
        return Err(NotApplicableError);
    }

    Ok(InteriorPoint {
        done: false,
        x: DVector::from_vec(initial_point),
        big_a: {
            todo!();
            let a = constraints.iter().map(|(a, _, _)| *a).flatten().copied();
            let mut big_a = DMatrix::from_row_iterator(n, m, a).resize_horizontally(n + m, 0.0);
            big_a.view_mut((0, n), (n, n)).fill_with_identity();
            big_a
        },
        c: DVector::from_vec(objective_function).resize_vertically(n + m, 0.0),
        eps: up_to_n_dec_places(i32::try_from(eps).map_err(|_| NotApplicableError)?),
        alpha,
    })
}

fn up_to_n_dec_places(n: i32) -> f64 {
    0.1_f64.powi(n) / 2.
}
