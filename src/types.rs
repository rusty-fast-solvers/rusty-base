//! This module contains some basic definitions used by everything else.

use ndarray::{Array1, Array2, ArrayView1, ArrayView2, ArrayBase, Axis, Ix2, Data};
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

// Our default type is the ndarray_linalg Scalar type
pub use ndarray_linalg::Scalar;

/// This trait specifies the required floating point properties for real types.
/// Currently, we support f32 and f64.
/// This type is specifically for operations that only make sense for real types.
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

// Complex types
pub use ndarray_linalg::c32;
pub use ndarray_linalg::c64;

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

/// Matrix-Vector Product Trait
///
/// This trait defines an interface for operators that provide matrix-vector products.
pub trait MatVec {
    type A: Scalar;

    // Return the number of rows of the operator.
    fn nrows(&self) -> usize;

    // Return the number of columns of the operator.
    fn ncols(&self) -> usize;

    // Return the matrix vector product of an operator with a vector.
    fn matvec(&self, mat: ArrayView1<Self::A>) -> Array1<Self::A>;
}

/// Matrix-Matrix Product Trait
///
/// This trait defines the application of a linear operator $A$ to a matrix X representing multiple columns.
/// If it is not implemented then a default implementation is used based on the `MatVec` trait applied to the
/// individual columns of X.
pub trait MatMat: MatVec {
    // Return the matrix-matrix product of an operator with a matrix.
    fn matmat(&self, mat: ArrayView2<Self::A>) -> Array2<Self::A> {
        let mut output = Array2::<Self::A>::zeros((self.nrows(), mat.ncols()));

        for (index, col) in mat.axis_iter(Axis(1)).enumerate() {
            output
                .index_axis_mut(Axis(1), index)
                .assign(&self.matvec(col));
        }

        output
    }
}

/// Trait describing the product of the conjugate adjoint of an operator with a vector
///
/// In the case that the operator is a matrix then this simply describes the action $A^Hx$,
/// where $x$ is a vector and $A^H$ the complex conjugate adjoint of $A$.
pub trait ConjMatVec: MatVec {
    // If `self` is a linear operator return the product of the conjugate of `self`
    // with a vector.
    fn conj_matvec(&self, vec: ArrayView1<Self::A>) -> Array1<Self::A>;
}

/// Trait describing the action of the conjugate adjoint of an operator with a matrix
///
/// In the case that the operator is a matrix then this simply describes the action $A^HX$,
/// where $X$ is another matrix and $A^H$ the complex conjugate adjoint of $A$. If this trait
/// is not implemented then a default implementation based on the `ConjMatVec` trait is used.
pub trait ConjMatMat: MatMat + ConjMatVec {
    // Return the product of the complex conjugate of `self` with a given matrix.
    fn conj_matmat(&self, mat: ArrayView2<Self::A>) -> Array2<Self::A> {
        let mut output = Array2::<Self::A>::zeros((self.nrows(), mat.ncols()));

        for (index, col) in mat.axis_iter(Axis(1)).enumerate() {
            output
                .index_axis_mut(Axis(1), index)
                .assign(&self.conj_matvec(col));
        }

        output
    }
}

impl<A, S> MatVec for ArrayBase<S, Ix2>
where
    A: Scalar,
    S: Data<Elem = A>,
{
    type A = A;

    fn nrows(&self) -> usize {
        self.nrows()
    }

    fn ncols(&self) -> usize {
        self.ncols()
    }

    fn matvec(&self, vec: ArrayView1<Self::A>) -> Array1<Self::A> {
        self.dot(&vec)
    }
}

impl<A, S> ConjMatVec for ArrayBase<S, Ix2>
where
    A: Scalar,
    S: Data<Elem = A>,
{
    fn conj_matvec(&self, vec: ArrayView1<Self::A>) -> Array1<Self::A> {
        vec.map(|item| item.conj())
            .dot(self)
            .map(|item| item.conj())
    }
}

impl<A, S> MatMat for ArrayBase<S, Ix2>
where
    A: Scalar,
    S: Data<Elem = A>,
{
    fn matmat(&self, mat: ArrayView2<Self::A>) -> Array2<Self::A> {
        self.dot(&mat)
    }
}

impl<A, S> ConjMatMat for ArrayBase<S, Ix2>
where
    A: Scalar,
    S: Data<Elem = A>,
{
    fn conj_matmat(&self, mat: ArrayView2<Self::A>) -> Array2<Self::A> {
        mat.t()
            .map(|item| item.conj())
            .dot(self)
            .t()
            .map(|item| item.conj())
    }
}
