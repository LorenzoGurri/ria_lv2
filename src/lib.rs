use lv2::prelude::*;

pub mod effects;
pub mod pipeline;
pub mod ri;

use effects::Effect;
use effects::{allpass, amplfo, echo, highpass, lowpass};
use std::collections::LinkedList;

#[derive(PortCollection)]
struct Ports {
    control: InputPort<Control>,
    input: InputPort<Audio>,
    output: OutputPort<Audio>,
}

#[uri("https://github.com/RustAudio/rust-lv2/tree/master/docs/amp")]
struct RiaLV2 {
    pline: pipeline::Pipeline,
}

impl Plugin for RiaLV2 {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = ();

    fn new(plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        let fs = plugin_info.sample_rate() as f32;
        let mut pline = pipeline::Pipeline::new(fs);

        // static example of filters in series
        let mut effects = LinkedList::new();
        effects.push_back(
            Box::new(lowpass::IIRLowPass::new(fs, 1000.)) as Box<dyn Effect + Send + Sync>
        );
        effects.push_back(
            Box::new(highpass::IIRHighPass::new(fs, 500.)) as Box<dyn Effect + Send + Sync>
        );
        effects.push_back(
            Box::new(echo::Echo::new(fs, 0.2, 10, echo::DecayType::Linear, 1.))
                as Box<dyn Effect + Send + Sync>,
        );
        pline.create(effects);
        Some(Self { pline })
    }

    fn run(&mut self, ports: &mut Ports, _features: &mut (), _: u32) {
        for (out_sample, in_sample) in Iterator::zip(ports.output.iter_mut(), ports.input.iter()) {
            *out_sample = self.pline.run(*in_sample, *ports.control);
        }
    }
}

lv2_descriptors!(RiaLV2);
