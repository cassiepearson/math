//! Numerical Traits - For more convenient trait bounding
use num::{Bounded, CheckedMul, FromPrimitive, Num, NumCast, ToPrimitive};
use rand::distributions::uniform::SampleUniform;
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

pub trait Number:
    Num
    + NumCast
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + ToPrimitive
    + FromPrimitive
    + SampleUniform
    + Send
    + Sync
    + Copy
{
}

impl<T> Number for T where
    T: Num
        + NumCast
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + ToPrimitive
        + FromPrimitive
        + SampleUniform
        + Send
        + Sync
        + Copy
{
}

pub trait Integer: Number + Bounded + Ord + CheckedMul {}

impl<T> Integer for T where T: Number + Bounded + Ord + CheckedMul {}

pub trait Float: Number + PartialOrd {}

impl<T> Float for T where T: Number + PartialOrd {}
