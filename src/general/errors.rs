//! Errors enum
use thiserror::Error;

/// Errors enumeration
///
/// Lists all possible errors that can occur during the execution of the program
#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum MathLibError {
    #[error("Math operation error:\n{0}\n")]
    NumberTheoryErr(String),
}
