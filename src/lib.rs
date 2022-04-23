use fs2::FileExt;
use lv2::lv2_worker::*;
use lv2::prelude::*;

use std::any::Any;
use std::collections::LinkedList;
use std::env;
use std::fs::File;

pub mod effects;
pub mod parser;
pub mod pipeline;

use effects::Effect;

// Collection of LV2 Ports Used
#[derive(PortCollection)]
struct Ports {
    control: InputPort<Control>,
    lin: InputPort<Audio>,
    rin: InputPort<Audio>,
    lout: OutputPort<Audio>,
    rout: OutputPort<Audio>,
}

// Requested features
#[derive(FeatureCollection)]
struct AudioFeatures<'a> {
    // request to schedule work
    schedule: Schedule<'a, RiaLV2>,
}

// RiaLV2 Plugin Struct
#[uri("https://github.com/LorenzoGurri/ria_lv2")]
struct RiaLV2 {
    fs: f32,
    pline: pipeline::Pipeline,
}

impl Plugin for RiaLV2 {
    type Ports = Ports;
    type InitFeatures = ();
    type AudioFeatures = AudioFeatures<'static>;

    fn new(plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
        let fs = plugin_info.sample_rate() as f32;
        let pline = pipeline::Pipeline::new();
        Some(Self { fs, pline })
    }

    fn run(&mut self, ports: &mut Ports, features: &mut Self::AudioFeatures, _: u32) {
        // schedule work if the plugin is active
        if *ports.control > 0. {
            if let Err(e) = features.schedule.schedule_work(self.fs) {
                println!("Can't schedule work: {}", e);
            }
        }

        let in_samples = Iterator::zip(ports.lin.iter(), ports.rin.iter());
        let out_samples = Iterator::zip(ports.lout.iter_mut(), ports.rout.iter_mut());

        // process samples with the effects in Pipeline
        for (in_data, out_data) in Iterator::zip(in_samples, out_samples) {
            let (left_in, right_in) = in_data;
            // let (left_out, right_out) = out_data;
            let (left_out, right_out) = self.pline.run(*left_in, *right_in, *ports.control);
            *out_data.0 = left_out;
            *out_data.1 = right_out;
        }
    }

    fn extension_data(uri: &Uri) -> Option<&'static dyn Any> {
        match_extensions![uri, WorkerDescriptor<Self>]
    }
}

impl Worker for RiaLV2 {
    // data type sent by the schedule handler and received by the `work` method.
    type WorkData = f32;
    // data type sent by the response handler and received by the `work_response` method.
    type ResponseData = Option<LinkedList<Box<dyn Effect + Send + Sync>>>;

    fn work(
        response_handler: &ResponseHandler<Self>,
        data: Self::WorkData,
    ) -> Result<(), WorkerError> {
        // open file and lock it
        let path;
        match env::var("RIA_PATH") {
            Ok(val) => path = val,
            Err(_) => match env::var("HOME") {
                Ok(val) => path = val,
                Err(_) => path = "".to_string(),
            },
        }
        let mut file = File::options()
            .read(true)
            .write(true)
            .open(path + "/rialv2_update.txt")
            .expect("Failed to open file");
        file.lock_exclusive().expect("Failed to obtain file lock");

        // parse file and return the list of effects specified in it
        let resp = parser::parse_file(&mut file, data);

        // unlock the file
        file.unlock().expect("Filed to unlock file");

        // send the effects list to work_response()
        if let Err(e) = response_handler.respond(resp) {
            eprintln!("Can't respond: {}", e);
        }
        Ok(())
    }

    fn work_response(
        &mut self,
        filters: Self::ResponseData,
        _features: &mut Self::AudioFeatures,
    ) -> Result<(), WorkerError> {
        // receive the effects from work()
        // if there are new filters, set them in the pipeline
        if let Some(f) = filters {
            self.pline.set(f);
        }
        Ok(())
    }

    fn end_run(&mut self, _features: &mut Self::AudioFeatures) -> Result<(), WorkerError> {
        Ok(())
    }
}

lv2_descriptors!(RiaLV2);
