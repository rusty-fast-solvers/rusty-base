//! This crate defines basic data structures and types used throughout
//! the other `rusty-fast-solver` creates.

pub mod particle_container;
pub mod core;

pub use crate::core::RealType;
pub use crate::core::KernelType;
pub use crate::core::ThreadingType;

pub use crate::particle_container::make_particle_container;
pub use crate::particle_container::make_particle_container_owned;

pub use crate::particle_container::ParticleContainerAccessor;
pub use crate::particle_container::ParticleContainer;
pub use crate::particle_container::ParticleContainerView;

