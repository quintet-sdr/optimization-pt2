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
    let (n, m) = get_n_and_m(constraints).ok_or(NotApplicableError)?;

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
        big_a: build_big_a(constraints),
        c: DVector::from_vec(objective_function).resize_vertically(n + m, 0.),
        eps: up_to_n_dec_places(i32::try_from(eps).map_err(|_| NotApplicableError)?),
        alpha,
    })
}

fn get_n_and_m(constraints: &[(&[f64], Sign, f64)]) -> Option<(usize, usize)> {
    Some((constraints.len(), constraints.first()?.0.len()))
}

fn build_big_a(constraints: &[(&[f64], Sign, f64)]) -> DMatrix<f64> {
    let (n, m) = get_n_and_m(constraints).unwrap();

    let left_part_row_elements = constraints
        .iter()
        .flat_map(|(coefficients, _, _)| *coefficients)
        .copied();

    let right_part_diagonal_elements = &DVector::from_vec(
        constraints
            .iter()
            .map(|(_, sign, _)| match sign {
                Sign::Le => 1.,
                Sign::Eq => 0.,
                Sign::Ge => -1.,
            })
            .collect(),
    );

    let mut big_a =
        DMatrix::from_row_iterator(n, m, left_part_row_elements).resize_horizontally(m + n, 0.);

    big_a
        .view_mut((0, m), (n, m))
        .set_diagonal(right_part_diagonal_elements);

    let no_slack_rows = constraints
        .iter()
        .enumerate()
        .filter_map(|(i, (_, sign, _))| matches!(sign, Sign::Eq).then_some(i));

    let no_slack_columns = no_slack_rows.map(|j| m + j).collect::<Box<[_]>>();

    big_a.remove_columns_at(&no_slack_columns)
}

fn up_to_n_dec_places(n: i32) -> f64 {
    0.1_f64.powi(n) / 2.
}
