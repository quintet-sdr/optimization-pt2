use interfaces::NotApplicableError;
use na::{DMatrix, DVector};

use crate::interfaces::InteriorPoint;

mod algorithm;
mod interfaces;

#[allow(clippy::many_single_char_names)]
pub fn interior_point(
    c: Vec<f64>,
    a: Vec<Vec<f64>>,
    initial_point: Vec<f64>,
    b: &[f64],
    alpha: f64,
    eps: usize,
) -> Result<InteriorPoint, NotApplicableError> {
    let n = a.len();
    let m = a.first().ok_or(NotApplicableError)?.len();

    if b.len() != n || a.iter().any(|row| row.len() != c.len()) || initial_point.len() != n + m {
        return Err(NotApplicableError);
    }

    Ok(InteriorPoint {
        done: false,
        x: DVector::from_vec(initial_point),
        big_a: {
            let a = a.into_iter().flatten();
            let mut big_a = DMatrix::from_row_iterator(n, m, a).resize_horizontally(n + m, 0.0);
            big_a.view_mut((0, n), (n, n)).fill_with_identity();
            big_a
        },
        c: DVector::from_vec(c).resize_vertically(n + m, 0.0),
        alpha,
        eps: 0.1_f64.powi(<i32>::try_from(eps).map_err(|_| NotApplicableError)? + 1),
    })
}
