//! # Low Pass Filter
//!
//! This module holds `IIRLowPass`, the
//! representation of a infinite impulse response
//! low pass filter.
//!
use crate::effects::Effect;
use biquad::*;

pub struct IIRLowPass {
    lbq: DirectForm2Transposed<f32>,
    rbq: DirectForm2Transposed<f32>,
}

/// The Low Pass Filter
///
/// The Low Pass Filter uses the crate
/// `biquad` to create a 2nd order IIR Low Pass
/// Filter.
impl IIRLowPass {
    /// # Params
    ///
    /// `fs` - the sample rate (e.g. 44100)
    ///
    /// `cutoff` - the cutoff frequency in Hz
    pub fn new(fs: f32, cutoff: f32) -> IIRLowPass {
        let coeffs = Coefficients::<f32>::from_params(
            Type::LowPass,
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

impl Effect for IIRLowPass {
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
