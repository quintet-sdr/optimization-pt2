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
    // c: Vec<f64>, a: Vec<Vec<f64>>, b: Vec<f64>, eps: i32
    // let a = DMatrix::from_row_slice(1, 3, &[1, 1, 1]);
    // let b = DMatrix::<f64>::identity(a.nrows(), a.ncols());

    // let c = vec![1, 2, 0];
    // let a = vec![1, 1, 1];
    // let b = 8;
}

fn interior_point_algorithm(
    initial_x: Vec<f64>,
    initial_a: Vec<Vec<f64>>,
    initial_c: Vec<f64>,
    alpha: f64,
    epsilon: f64,
) {
    let mut x = DVector::from_vec(initial_x);
    let a = DMatrix::from_vec(
        initial_a.len(),
        initial_a.first().unwrap().len(),
        initial_a.into_iter().flatten().collect(),
    );
    let c = DVector::from_vec(initial_c);

    let mut iteration = 1;
    loop {
        if is_inapplicable(&a, &c) {
            println!("Not applicable");
            break;
        }

        let d = Matrix::from_diagonal(&x);
        let aa = &a * &d;
        let cc = &d * &c;
        let i = DMatrix::<f64>::identity(c.nrows(), c.nrows());
        let f = &aa * aa.transpose();
        let fi = f.try_inverse().unwrap();
        let h = aa.tr_mul(&fi);
        let p = i - h * aa;
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

fn is_inapplicable(a: &DMatrix<f64>, c: &DVector<f64>) -> bool {
    c.iter()
        .enumerate()
        .filter(|(_, it)| it >= &&0.0)
        .any(|(i, _)| a.column(i).iter().all(|it| it <= &&0.0))
}
