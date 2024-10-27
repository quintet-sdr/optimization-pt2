use interfaces::NoSolutionError;
use na::{DMatrix, DVector};

use self::interfaces::InteriorPoint;

mod algorithm;
mod interfaces;

pub fn solve(
    c: Vec<f64>,
    a: Vec<Vec<f64>>,
    initial_point: Vec<f64>,
    b: Vec<f64>,
    alpha: f64,
    eps: usize,
) -> Result<InteriorPoint, NoSolutionError> {
    let n = a.len();
    let m = a.first().unwrap().len();

    assert_eq!(b.len(), n);
    a.iter().for_each(|row| assert_eq!(row.len(), c.len()));
    assert_eq!(initial_point.len(), n + m);

    let x = DVector::from_vec(initial_point);

    let big_a = {
        let a = a.into_iter().flatten();
        let mut big_a = DMatrix::from_row_iterator(n, m, a).resize_horizontally(n + m, 0.0);
        big_a.view_mut((0, n), (n, n)).fill_with_identity();
        big_a
    };
    let c = DVector::from_vec(c).resize_vertically(n + m, 0.0);

    Ok(InteriorPoint {
        done: false,
        x,
        big_a,
        c,
        alpha,
        eps: 0.1_f64.powi(eps as i32 + 1),
    })
}

pub fn run() {
    const EPS: usize = 2;

    let _ = solve(
        vec![9., 10., 16.],
        vec![vec![18., 15., 12.], vec![6., 4., 8.], vec![5., 3., 3.]],
        vec![1., 1., 1., 315., 174., 169.],
        vec![360., 192., 180.],
        0.5,
        EPS,
    );
}
