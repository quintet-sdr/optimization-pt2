use na::{DMatrix, DVector, Matrix};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("method is not applicable")]
    NotApplicable,
    #[error("problem has no solution")]
    NoSolution,
}

pub struct Solution {
    /// A vector of the decision variables.
    _x: Vec<f64>,
    /// The maximum value of the objective function.
    _z: f64,
}

pub fn run() {
    interior_point_algorithm(
        vec![2., 2., 4., 3.],
        vec![
            vec![2., -2., 8., 0.].into_boxed_slice(),
            vec![-6., -1., 0., -1.].into_boxed_slice(),
        ]
        .into_boxed_slice(),
        vec![-2., 3., 0., 0.],
        0.5,
        0.0001,
    )
}

fn interior_point_algorithm(
    initial_x: Vec<f64>,
    initial_a: Box<[Box<[f64]>]>,
    initial_c: Vec<f64>,
    alpha: f64,
    epsilon: f64,
) {
    let mut x = DVector::from_vec(initial_x);
    let a = DMatrix::from_vec(
        initial_a.first().unwrap().len(),
        initial_a.len(),
        IntoIterator::into_iter(initial_a).flatten().collect(),
    )
    .transpose();
    let c = DVector::from_vec(initial_c);

    if cfg!(debug_assertions) {
        println!("x:{x}");
        println!("a:{a}");
        println!("c:{c}");
    }

    let mut iteration = 1;
    loop {
        let d = Matrix::from_diagonal(&x);

        let aa = &a * &d;
        let cc = &d * &c;

        let i = DMatrix::<f64>::identity(c.nrows(), c.nrows());

        let f = &aa * aa.transpose();
        let fi = f.try_inverse().unwrap();
        let h = aa.tr_mul(&fi);

        let p = i - (h * aa);

        let cp = p * cc;

        let nu = cp.min().abs();
        let y = DVector::from_element(c.nrows(), 1.0) + (alpha / nu) * cp;

        let yy = d * y;

        iteration += 1;

        if (&yy - x).norm() < epsilon {
            x = yy;

            println!("In the last iteration {iteration}, we have x =");
            println!("{x}");
            println!("with alpha = {alpha}.");
            println!("Value of objective function is: {}", c * x.transpose());
            break;
        }

        x = yy;
    }
}
