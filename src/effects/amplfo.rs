//! # Amplitude Low Frequency Oscillator Filter
//!
//! This module holds `AmpLFO`, the
//! representation of a Low Frequency Oscillator that
//! modulates the amplitude of the input audio.

use crate::effects::Effect;

/// The Amplitude Low Frequency Oscillator Filter
///
/// The Amplitude Low Frequency Oscillator Filter
/// oscillates between zero and one inclusive `[0,1]`
/// at a rate of `freq`.
/// It modulates the amplitude of the `in_sample`.
pub struct AmpLFO {
    fs: f32,
    freq: f32,
    x: f32,
}

impl AmpLFO {
    /// # Params
    ///
    /// `fs` - sample rate (e.g 44100)
    ///
    /// `freq` - frequency of the LFO in Hz
    pub fn new(fs: f32, freq: f32) -> AmpLFO {
        Self { fs, freq, x: 0. }
    }
}

impl Effect for AmpLFO {
    /// # Returns
    ///
    /// The modulated `in_sample` according to where
    /// the LFO is on its sinusoid.
    fn run(&mut self, in_samples: (f32, f32)) -> (f32, f32) {
        let out = ((2. * std::f32::consts::PI * self.x * self.freq / self.fs).cos() + 1.) / 2.;
        self.x = (self.x + 1.) % (self.fs / self.freq);
        (out * in_samples.0, out * in_samples.1 )
    }
}
