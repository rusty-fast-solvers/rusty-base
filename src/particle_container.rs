//! This module defines a simple container structure to store source particles and target particles
//! in a convenient way.

use ndarray::{Array2, ArrayView2};
use crate::RealType;

// This traits describes any type that provides an array of sources and
// an array of targets.
pub trait ParticleContainerAccessor {
    type A: RealType;

    /// Return a non-owning representation of the sources.
    fn sources(&self) -> ArrayView2<Self::A>;
    /// Return a non-owning representation of the targets.
    fn targets(&self) -> ArrayView2<Self::A>;
}

/// The basic data structure for sources and targets that are owned.
pub struct ParticleContainer<T: RealType> {
    sources: Array2<T>,
    targets: Array2<T>,
}

/// Create a new particle container and transfer ownership to it.
pub fn make_particle_container_owned<T: RealType>(
    sources: Array2<T>,
    targets: Array2<T>,
) -> ParticleContainer<T> {
    ParticleContainer { sources, targets }
}

/// Create a new particle container that does not take over ownership.
pub fn make_particle_container<'a, T: RealType>(
    sources: ArrayView2<'a, T>,
    targets: ArrayView2<'a, T>,
) -> ParticleContainerView<'a, T> {
    ParticleContainerView { sources, targets }
}

// The basic data structure of for sources and targets that are not owned.
pub struct ParticleContainerView<'a, T: > {
    sources: ArrayView2<'a, T>,
    targets: ArrayView2<'a, T>,
}

impl<A: RealType> ParticleContainerAccessor for ParticleContainer<A> {
    type A = A;

    /// Return a non-owning representation of the sources.
    fn sources(&self) -> ArrayView2<Self::A> {
        self.sources.view()
    }

    /// Return a non-owning representation of the targets.
    fn targets(&self) -> ArrayView2<Self::A> {
        self.targets.view()
    }
}

impl<'a, A: RealType> ParticleContainerAccessor for ParticleContainerView<'a, A> {
    type A = A;

    /// Return a non-owning representation of the sources.
    fn sources(&self) -> ArrayView2<Self::A> {
        self.sources.view()
    }

    /// Return a non-owning representation of the targets.
    fn targets(&self) -> ArrayView2<Self::A> {
        self.targets.view()
    }
}
