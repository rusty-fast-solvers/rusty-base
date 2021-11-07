//! This crate defines basic data structures and types used throughout
//! the other `rusty-fast-solver` crates.


pub use ndarray_linalg::Scalar;

pub mod particle_container;
pub mod types;

pub use crate::types::RealType;
pub use crate::types::KernelType;
pub use crate::types::ThreadingType;
pub use crate::types::EvalMode;
pub use crate::types::Result;

pub use crate::types::MatVec;
pub use crate::types::MatMat;
pub use crate::types::ConjMatVec;
pub use crate::types::ConjMatMat;

pub use crate::particle_container::make_particle_container;
pub use crate::particle_container::make_particle_container_owned;

pub use crate::particle_container::ParticleContainerAccessor;
pub use crate::particle_container::ParticleContainer;
pub use crate::particle_container::ParticleContainerView;


