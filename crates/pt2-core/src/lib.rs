use na::{DMatrix, DVector};

pub use crate::interfaces::Sign;
use crate::interfaces::{InteriorPoint, NotApplicableError};

mod algorithm;
mod interfaces;

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

    let x = DVector::from_vec(initial_point);
    let big_a = {
        let left_part_row_elements = constraints
            .iter()
            .flat_map(|(coefficients, _, _)| *coefficients)
            .copied();

        let right_part_diagonal_elements = &DVector::from_vec(
            constraints
                .iter()
                .filter_map(|(_, sign, _)| match sign {
                    Sign::Le => Some(1.),
                    Sign::Ge => Some(-1.),
                    Sign::Eq => None,
                })
                .collect(),
        );

        let mut big_a = DMatrix::from_row_iterator(n, m, left_part_row_elements)
            .resize_horizontally(n + right_part_diagonal_elements.len(), 0.);

        big_a
            .view_mut((0, n), (n, right_part_diagonal_elements.len()))
            .set_diagonal(right_part_diagonal_elements);

        big_a
    };
    let c = DVector::from_vec(objective_function).resize_vertically(n + m, 0.);
    let eps = up_to_n_dec_places(i32::try_from(eps).map_err(|_| NotApplicableError)?);

    Ok(InteriorPoint {
        done: false,
        x,
        big_a,
        c,
        eps,
        alpha,
    })
}

fn up_to_n_dec_places(n: i32) -> f64 {
    0.1_f64.powi(n) / 2.
}
