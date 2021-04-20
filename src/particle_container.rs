//! This module defines a simple container structure to store source particles and target particles
//! in a convenient way.

use crate::RealType;
use ndarray::{Array2, ArrayView2};

// This traits describes any type that provides an array of sources and
// an array of targets.
pub trait ParticleContainerAccessor {
    type FloatingPointType: RealType;

    /// Return a non-owning representation of the sources.
    fn sources(&self) -> ArrayView2<Self::FloatingPointType>;
    /// Return a non-owning representation of the targets.
    fn targets(&self) -> ArrayView2<Self::FloatingPointType>;
}

/// The basic data structure of this library for sources and targets
/// that are owned by the structure.
pub struct ParticleContainer<T: RealType> {
    sources: Array2<T>,
    targets: Array2<T>,
}

/// Create a new particle container
pub fn make_particle_container_owned<T: RealType>(
    sources: Array2<T>,
    targets: Array2<T>,
) -> ParticleContainer<T> {
    ParticleContainer { sources, targets }
}

pub fn make_particle_container<'a, T: RealType>(
    sources: ArrayView2<'a, T>,
    targets: ArrayView2<'a, T>,
) -> ParticleContainerView<'a, T> {
    ParticleContainerView { sources, targets }
}

// The basic data structure of this library for sources and targets
// that are not owned by the structure.
pub struct ParticleContainerView<'a, T: RealType> {
    sources: ArrayView2<'a, T>,
    targets: ArrayView2<'a, T>,
}

impl<T: RealType> ParticleContainerAccessor for ParticleContainer<T> {
    type FloatingPointType = T;

    /// Return a non-owning representation of the sources.
    fn sources(&self) -> ArrayView2<Self::FloatingPointType> {
        self.sources.view()
    }

    /// Return a non-owning representation of the targets.
    fn targets(&self) -> ArrayView2<Self::FloatingPointType> {
        self.targets.view()
    }
}

impl<'a, T: RealType> ParticleContainerAccessor for ParticleContainerView<'a, T> {
    type FloatingPointType = T;

    /// Return a non-owning representation of the sources.
    fn sources(&self) -> ArrayView2<Self::FloatingPointType> {
        self.sources.view()
    }

    /// Return a non-owning representation of the targets.
    fn targets(&self) -> ArrayView2<Self::FloatingPointType> {
        self.targets.view()
    }
}
