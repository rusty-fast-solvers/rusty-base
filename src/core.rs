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

pub enum KernelType {
    Laplace,
    Helmholtz(Complex<f64>),
    ModifiedHelmholtz(f64),
}

pub enum ThreadingType {
    Parallel,
    Serial,
}
