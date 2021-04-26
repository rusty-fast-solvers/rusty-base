//! This module contains some basic definitions used by everything else.

use num;
use num::complex::Complex;

/// This trait specifies the required floating point properties for real types.
/// Currently, we support f32 and f64.
pub trait RealType:
    num::traits::NumAssignOps
    + std::marker::Send
    + std::marker::Sync
    + num::traits::Float
    + num::traits::FloatConst
    + std::fmt::Display
{
}

impl RealType for f32 {}
impl RealType for f64 {}

/// This enum defines the type of the kernel.
pub enum KernelType {
    /// The Laplace kernel defined as g(x, y) = 1 / (4 pi | x- y| )
    Laplace,
    /// The Helmholtz kernel defined as g(x, y) = exp( 1j * k * | x- y| ) / (4 pi | x- y| )
    Helmholtz(Complex<f64>),
    /// The modified Helmholtz kernel defined as g(x, y) = exp( -omega * | x- y| ) / (4 * pi * | x- y |)
    ModifiedHelmholtz(f64),
}

/// Determines whether to use multithreading or serial evaluation.
pub enum ThreadingType {
    /// Use multithreading
    Parallel,
    /// Use serial evaluation
    Serial,
}
