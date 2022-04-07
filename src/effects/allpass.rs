//! # All Pass Filter
//!
//! This module holds `AllPass`, the
//! representation of an all pass filter.

use crate::effects::Effect;

/// The All Pass Filter
///
/// The All Pass filter simply
/// returns the input sample `in_sample`
/// it was given.
pub struct AllPass;

impl Effect for AllPass {
    /// # Returns
    ///
    /// The input sample `in_sample`.
    fn run(&mut self, in_sample: f32) -> f32 {
        in_sample
    }
}
