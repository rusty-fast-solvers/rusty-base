//! This module contains some basic definitions used by everything else.

use ndarray_linalg;
use num;
use num::complex::Complex;

/// This enum defines the type of the kernel.
pub enum KernelType {
    /// The Laplace kernel defined as g(x, y) = 1 / (4 pi | x- y| )
    Laplace,
    /// The Helmholtz kernel defined as g(x, y) = exp( 1j * k * | x- y| ) / (4 pi | x- y| )
    Helmholtz(Complex<f64>),
    /// The modified Helmholtz kernel defined as g(x, y) = exp( -omega * | x- y| ) / (4 * pi * | x- y |)
    ModifiedHelmholtz(f64),
}

/// This trait specifies the required floating point properties for real types.
/// Currently, we support f32 and f64.
pub trait RealType:
    std::marker::Send
    + std::marker::Sync
    + num::traits::Float
    + num::traits::FloatConst
    + ndarray_linalg::Scalar
{
}

impl<T: Send + Sync + num::traits::Float + num::traits::FloatConst + ndarray_linalg::Scalar>
    RealType for T
{
}

/// Determines whether to use multithreading or serial evaluation.
pub enum ThreadingType {
    /// Use multithreading
    Parallel,
    /// Use serial evaluation
    Serial,
}

/// This enum defines the Evaluation Mode for kernels.
pub enum EvalMode {
    /// Only evaluate Green's function values.
    Value,
    /// Evaluate values and derivatives.
    ValueGrad,
}

pub type Result<T> = std::result::Result<T, &'static str>;
