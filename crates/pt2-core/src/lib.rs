use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use na::{DMatrix, DVector};

use self::error::{NoSolutionError, NotApplicableError};

mod error;

pub struct Solution {
    // /// A vector of the decision variables.
    // x: Vec<f64>,
    // /// The maximum value of the objective function.
    // z: f64,
    x: Weak<RefCell<DVector<f64>>>,
    objective_fn: DMatrix<f64>,
}

pub fn run() {
    let a = solve(
        vec![2., 2., 4., 3.],
        vec![vec![2., -2., 8., 0.], vec![-6., -1., 0., -1.]],
        vec![-2., 3., 0., 0.],
        0.5,
        3,
    );
}

struct Ipa {
    x: Rc<RefCell<DVector<f64>>>,
    a: DMatrix<f64>,
    c: DVector<f64>,
    alpha: f64,
    eps: u8,
    done: bool,
}

impl Iterator for Ipa {
    type Item = Result<Solution, NotApplicableError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let d = DMatrix::from_diagonal(&self.x.borrow());

        let aa = &self.a * &d;
        let cc = &d * &self.c;

        let i = DMatrix::<f64>::identity(self.c.nrows(), self.c.nrows());

        let f = &aa * aa.transpose();
        let Some(fi) = f.try_inverse() else {
            return Some(Err(NotApplicableError));
        };
        let h = aa.tr_mul(&fi);

        let p = i - (h * aa);

        let cp = p * cc;

        let nu = cp.min().abs();
        let y = DVector::from_element(self.c.nrows(), 1.0) + (self.alpha / nu) * cp;

        let yy = d * y;
        let norm = (&yy - &*self.x.borrow()).norm();
        *self.x.borrow_mut() = yy;

        if norm < 0.1_f64.powi(self.eps.into()) {
            self.done = true;
        }

        Some(Ok(Solution {
            x: Rc::downgrade(&self.x),
            objective_fn: &self.c * self.x.borrow().transpose(),
        }))
    }
}

pub fn solve(
    x: Vec<f64>,
    a: Vec<Vec<f64>>,
    c: Vec<f64>,
    alpha: f64,
    eps: u8,
) -> Result<Ipa, NoSolutionError> {
    // if !todo!() {
    //     return Err(NoSolutionError);
    // }

    Ok(Ipa {
        x: Rc::new(RefCell::new(DVector::from_vec(x))),
        a: DMatrix::from_row_iterator(a.len(), a.first().unwrap().len(), a.into_iter().flatten()),
        c: DVector::from_vec(c),
        alpha,
        eps,
        done: false,
    })
}

// pub fn main(c: Vec<f64>, a: Vec<Vec<f64>>, initial_point: Vec<f64>, b: Vec<f64>, eps: u8) {
//     assert_eq!(a.len(), b.len());
//     a.iter().for_each(|row| assert_eq!(row.len(), c.len()));
//     todo!();
// }
