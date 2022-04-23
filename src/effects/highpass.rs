//! # High Pass Filter
//!
//! This module holds `IIRHighPass`, the
//! representation of a infinite impulse response
//! high pass filter.

use crate::effects::Effect;
use biquad::*;

/// The High Pass Filter
///
/// The High Pass Filter uses the crate
/// `biquad` to create a 2nd order IIR High Pass
/// Filter.
pub struct IIRHighPass {
    lbq: DirectForm2Transposed<f32>,
    rbq: DirectForm2Transposed<f32>,
}

impl IIRHighPass {
    /// # Params
    ///
    /// `fs` - the sample rate (e.g. 44100)
    ///
    /// `cutoff` - the cutoff frequency in Hz
    pub fn new(fs: f32, cutoff: f32) -> IIRHighPass {
        let coeffs = Coefficients::<f32>::from_params(
            Type::HighPass,
            fs.hz(),
            cutoff.hz(),
            Q_BUTTERWORTH_F32,
        )
        .unwrap();

        Self {
            lbq: DirectForm2Transposed::<f32>::new(coeffs),
            rbq: DirectForm2Transposed::<f32>::new(coeffs),
        }
    }
}

impl Effect for IIRHighPass {
    /// # Returns
    ///
    /// The modulated `in_sample` that
    /// corresponds to the following difference
    /// equation:
    ///
    /// `y[n] = b0*x[n] + b1*x[n-1] + b2*x[n-2] + a1*y[n-1] + a2*y[n-2]`
    ///
    /// where the `a`s and `b`s are calculated based on the `cutoff` frequency.
    fn run(&mut self, in_samples: (f32, f32)) -> (f32, f32) {
        (self.lbq.run(in_samples.0), self.rbq.run(in_samples.1))
    }
}
