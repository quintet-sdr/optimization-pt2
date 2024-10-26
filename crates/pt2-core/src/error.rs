use thiserror::Error;

#[derive(Error, Debug)]
#[error("method is not applicable")]
pub struct NotApplicableError;

#[derive(Error, Debug)]
#[error("problem has no solution")]
pub struct NoSolutionError;
