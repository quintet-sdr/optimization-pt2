use na::{DMatrix, DVector};

use crate::interfaces::{Auxiliary, InteriorPoint, Iteration, NoSolutionError};

impl Iterator for InteriorPoint {
    type Item = Result<Iteration, NoSolutionError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let size = self.x.len();

        let big_d = DMatrix::from_diagonal(&self.x);

        let big_a_tilde = &self.big_a * &big_d;
        let c_tilde = &big_d * &self.c;

        let big_p = {
            let big_i = DMatrix::identity(size, size);
            let big_a_tilde_tr = big_a_tilde.transpose();
            let Some(inverse) = (&big_a_tilde * &big_a_tilde_tr).try_inverse() else {
                return Some(Err(NoSolutionError));
            };
            big_i - big_a_tilde_tr * inverse * &big_a_tilde
        };
        let c_p = &big_p * &c_tilde;

        let Some(nu) = c_p
            .iter()
            .filter(|it| it < &&0.)
            .map(|it| it.abs())
            .max_by(|a, b| a.partial_cmp(b).unwrap())
        else {
            return Some(Err(NoSolutionError));
        };
        let x_tilde = DVector::from_element(size, 1.) + (self.alpha / nu) * &c_p;

        let new_x = &big_d * &x_tilde;
        if (&new_x - &self.x).norm() < self.eps {
            self.done = true;
        }

        self.x = new_x;

        Some(Ok(Iteration {
            auxiliary: Auxiliary {
                big_d,
                big_a_tilde,
                c_tilde,
                big_p,
                c_p,
                nu,
                x_tilde,
            },
            decision_variables: self.x.clone_owned(),
            max: self.x.dot(&self.c),
        }))
    }
}
