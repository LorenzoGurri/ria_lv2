//! # The Effects module. Holds the implementation of the effects/filters.
//!
//! This module holds the different effects that
//! are implemented as well as the trait `Effect`
//! that they all must implement.

/// # Effect Trait
///
/// This trait is implemented by all the effects in this
/// module. It allows for dynamic disbatch and thus makes it
/// easier to combine different effects in series at the cost
/// of a small performance hit.
pub trait Effect {
    fn run(&mut self, in_samples: (f32, f32)) -> (f32, f32);
}

pub mod allpass;
pub mod amplfo;
pub mod echo;
pub mod highpass;
pub mod lowpass;
