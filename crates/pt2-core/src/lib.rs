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

pub fn interior_point(
    c: Vec<f64>,
    a: Vec<Vec<f64>>,
    b: Vec<f64>,
    eps: i32,
) -> Result<Solution, Error> {
    todo!()
}
