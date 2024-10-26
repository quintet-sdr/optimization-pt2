use thiserror::Error;

#[derive(Error, Debug)]
#[error("method is not applicable")]
pub struct NotApplicable;

#[derive(Error, Debug)]
#[error("problem has no solution")]
pub struct NoSolution;
