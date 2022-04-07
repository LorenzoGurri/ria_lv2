//! # Echo Filter
//!
//! This module holds `Echo`, the
//! representation of an echo filter.

use crate::effects::Effect;
use std::collections::LinkedList;

// A delay specifit to the Echo effect
struct Delay {
    // sample rate in Hz
    fs: f32,
    // delay in seconds
    delay: f32,
    // buffer of samples
    buffer: LinkedList<f32>,
}

impl Delay {
    fn new(fs: f32, delay: f32) -> Delay {
        Self {
            fs,
            delay,
            buffer: LinkedList::new(),
        }
    }

    // Push samples into `buffer` until it's full,
    //   then start returning them
    fn run(&mut self, in_sample: f32) -> f32 {
        if self.buffer.len() < (self.delay * self.fs) as usize {
            self.buffer.push_back(in_sample);
            return in_sample;
        }
        self.buffer.pop_front().unwrap_or(0.)
    }
}

/// The type of decay used in the `Echo` struct.
pub enum DecayType {
    /// Linear decay of echoes
    ///
    /// `coef = start_decay_coef - x / (num_echoes + 1)`
    ///
    /// where `x` is the nth echo
    ///
    /// `start_decay_ceof` is between 0 and 1 inclusive
    ///
    /// `num_echoes` is the total number of echoes
    Linear,
    /// Exponential decay of echoes
    ///
    /// `coef = start_decay_coef * (1. - 2. / (num_echoes as f32 + 2.))^x`
    ///
    /// where `x` is the nth echo
    ///
    /// `start_decay_ceof` is between 0 and 1 inclusive
    ///
    /// `num_echoes` is the total number of echoes
    Exponential,
}

/// The Echo Filter
///
/// The Echo Filter uses a number of delays
/// to give the effect of an echo.
///
/// The echoes will gradually decay over
/// time according to the type of decay (`DecayType::Linear`
/// or `DecayType::Exponential`).
pub struct Echo {
    delays: Vec<Delay>,
    decays: Vec<f32>,
}

impl Echo {
    /// # Params
    ///
    /// `fs` - the sample rate (e.g. 44100)
    ///
    /// `delay` - the delay in seconds
    ///
    /// `num_echoes` - number of echoes we want
    ///
    /// `decay_type` - the way we want the echoes to die out over time
    ///
    /// `start_decay_coef` - how loud we want the first echo to be
    pub fn new(
        fs: f32,
        delay: f32,
        num_echoes: u8,
        decay_type: DecayType,
        start_decay_coef: f32,
    ) -> Echo {
        let decay_exponential = |x: f32| {
            (start_decay_coef * (1. - 2. / (num_echoes as f32 + 2.)).powi(x as i32)) as f32
        };

        let decay_linear = |x: f32| (start_decay_coef - x / (num_echoes + 1) as f32);

        let delays = (1..=num_echoes)
            .map(|x| Delay::new(fs, delay * x as f32))
            .collect();

        let decays = match decay_type {
            DecayType::Exponential => (1..=num_echoes)
                .map(|x| decay_exponential(x as f32))
                .collect(),
            DecayType::Linear => (1..=num_echoes).map(|x| decay_linear(x as f32)).collect(),
        };

        Self { delays, decays }
    }
}

impl Effect for Echo {
    /// # Returns
    ///
    /// The `in_sample` plus the delays we have
    /// added to it to make an echo effect.
    fn run(&mut self, in_sample: f32) -> f32 {
        let mut out_sample: f32 = in_sample;

        for i in 0..self.delays.len() {
            out_sample += self.decays[i] * self.delays[i].run(in_sample);
        }
        out_sample
    }
}
