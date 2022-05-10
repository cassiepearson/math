//! Errors enum
use std::string::FromUtf8Error;
use thiserror::Error;

//TODO: Update with cryptography errors

/// Library errors enumeration
///
/// Lists all possible errors that can occur in the Library
#[allow(dead_code)]
#[derive(Debug)]
pub enum MathLibErrors {
    NumberTheoryErr,
}

/// Number theory errors
///
/// Errors within the number theory module
#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum NumberTheoryErr {
    #[error("Operation overflow error:\n{0}\n")]
    Overflow(String),
    #[error("Inverse does not exist:\n{0}\n")]
    InverseDNE(String),
}

/// 2D Geometry errors
///
/// Errors within 2D geometry
#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum Geometry2DErr {
    // TODO: Fill out as needed
}

/// 3D Geometry errors
///
/// Errors within 2D geometry
#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum Geometry3DErr {
    // TODO: Fill out as needed
}

/// Cryptography errors
///
/// Errors within the classical cryptography module
#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum ClassicalCryptographyErr {
    #[error("Error creating string from Utf8 bytes : \n{0}\n")]
    FromUtf8Err(#[from] FromUtf8Error),
    #[error("Invalid affine cipher key. The gcd of the alpha, {0} and the max value of u8, {1} is not 1")]
    AffineKeyError(u8, u8),
}
