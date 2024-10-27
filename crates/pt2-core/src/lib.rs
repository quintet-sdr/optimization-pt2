use interfaces::NotApplicableError;
use na::{DMatrix, DVector};

use crate::interfaces::InteriorPoint;

mod algorithm;
mod interfaces;

#[allow(clippy::many_single_char_names)]
pub fn interior_point(
    c: Vec<f64>,
    a: &[&[f64]],
    initial_point: Vec<f64>,
    b: &[f64],
    eps: usize,
    alpha: f64,
) -> Result<InteriorPoint, NotApplicableError> {
    let n = a.len();
    let m = a.first().ok_or(NotApplicableError)?.len();

    if b.len() != n || initial_point.len() != n + m || a.iter().any(|row| row.len() != c.len()) {
        return Err(NotApplicableError);
    }
    let initial_point_is_feasible = a.iter().zip(b).all(|(constraint_factors, rhs)| {
        let constraint_sum: f64 = constraint_factors
            .iter()
            .zip(&initial_point)
            .map(|(factor, x)| factor * x)
            .sum();

        &constraint_sum <= rhs
    });
    if !initial_point_is_feasible {
        return Err(NotApplicableError);
    }

    Ok(InteriorPoint {
        done: false,
        x: DVector::from_vec(initial_point),
        big_a: {
            let a = a.iter().copied().flatten().copied();
            let mut big_a = DMatrix::from_row_iterator(n, m, a).resize_horizontally(n + m, 0.0);
            big_a.view_mut((0, n), (n, n)).fill_with_identity();
            big_a
        },
        c: DVector::from_vec(c).resize_vertically(n + m, 0.0),
        eps: up_to_n_dec_places(i32::try_from(eps).map_err(|_| NotApplicableError)?),
        alpha,
    })
}

fn up_to_n_dec_places(n: i32) -> f64 {
    0.1_f64.powi(n) / 2.
}
