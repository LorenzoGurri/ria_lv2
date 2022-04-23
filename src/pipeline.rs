//! # The Filter Pipeline. Allows for combining effects/filters.
//!
//! This module holds the struct `Pipeline`.
//! It offers the infrastructure to combine
//! effects and run them in series.

use crate::effects::allpass::AllPass;
use crate::effects::Effect;
use std::collections::LinkedList;

/// Pipeline
///
/// Pipeline allows for effects to be combined in
/// series. It does this through dynamic disbatch
/// and thus trades a small performance hit for
/// easier usage and simpler code.
pub struct Pipeline {
    effects: LinkedList<Box<dyn Effect + Send + Sync>>,
}

impl Pipeline {
    /// # Params
    ///
    /// `fs` - The sample rate (e.g. 44100)
    pub fn new() -> Pipeline {
        let mut effects = LinkedList::new();
        effects.push_back(Box::new(AllPass) as Box<dyn Effect + Send + Sync>);
        Self { effects }
    }

    /// `create` takes a vector of effects and puts them in series.
    ///
    /// # Params
    ///
    /// `effects` - The collection of effects we want to use.
    /// Note that they must be cast to a Box<dyn Effect + Send + Sync>>.
    pub fn set(&mut self, effects: LinkedList<Box<dyn Effect + Send + Sync>>) {
        self.effects = effects;
    }

    /// `run` passes the input sample `in_sample` through each effect.
    ///
    /// # Params
    ///
    /// `in_sample` - The input sample
    pub fn run(&mut self, left_sample: f32, right_sample: f32, toggle: f32) -> (f32, f32) {
        if toggle <= 0. {
            return (left_sample, right_sample);
        }

        let mut out: (f32, f32) = (left_sample, right_sample);
        for effect in &mut self.effects {
            out = effect.run(out);
        }
        out
    }
}
