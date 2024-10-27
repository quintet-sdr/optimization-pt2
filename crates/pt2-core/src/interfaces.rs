use std::cmp::Ordering;

use na::{DMatrix, DVector};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("method is not applicable")]
pub struct NotApplicableError;

#[derive(Error, Debug)]
#[error("problem has no solution")]
pub struct NoSolutionError;

pub struct Auxiliary {
    pub big_d: DMatrix<f64>,
    pub big_a_tilde: DMatrix<f64>,
    pub c_tilde: DVector<f64>,
    pub big_p: DMatrix<f64>,
    pub c_p: DVector<f64>,
    pub nu: f64,
    pub x_tilde: DVector<f64>,
}

pub struct Iteration {
    pub auxiliary: Auxiliary,
    pub x: DVector<f64>,
    pub max: f64,
}

pub struct InteriorPoint {
    pub(crate) done: bool,
    pub(crate) x: DVector<f64>,
    pub(crate) big_a: DMatrix<f64>,
    pub(crate) c: DVector<f64>,
    pub(crate) eps: f64,
    pub(crate) alpha: f64,
}

pub enum Sign {
    Le,
    Eq,
    Ge,
}

impl Sign {
    pub fn compare<T: PartialOrd>(&self, a: &T, b: &T) -> bool {
        let cmp_function = match self {
            Self::Le => PartialOrd::le,
            Self::Eq => PartialEq::eq,
            Self::Ge => PartialOrd::ge,
        };

        cmp_function(a, b)
    }
}
