use serde::{Deserialize, Serialize};
use serde_json;

use std::collections::LinkedList;
use std::fs::File;
use std::io::prelude::*;
use std::string::String;

use crate::effects::echo::DecayType;
use crate::effects::Effect;
use crate::effects::{allpass, amplfo, echo, highpass, lowpass};

#[derive(Serialize, Deserialize)]
struct EffectsInJson {
    effects: Vec<EffectInJson>,
}

#[derive(Serialize, Deserialize)]
struct EffectInJson {
    effect: String,
    params: Vec<f32>,
}

// NOTE: parse_file assumes we have the file lock
pub fn parse_file(file: &mut File, fs: f32) -> Option<LinkedList<Box<dyn Effect + Send + Sync>>> {
    // the list of effects
    let mut resp: LinkedList<Box<dyn Effect + Send + Sync>> = LinkedList::new();

    // if the file is empty, return
    if file.metadata().unwrap().len() == 0 {
        return None;
    }

    // read lines of file into string
    let mut lines = String::new();
    if let Err(e) = file.read_to_string(&mut lines) {
        eprintln!("Error reading lines: {}", e);
        return None;
    }

    // Parse the file as json
    let jsoneffects: EffectsInJson = match serde_json::from_str(lines.as_str()) {
        Ok(eff) => eff,
        Err(e) => {
            eprintln!("Error: {}", e);
            // zero out the file so we don't re-parse it
            file.set_len(0).expect("Failed to overwrite");
            return None;
        }
    };

    // for each effect, parse its parameters
    for jsoneffect in jsoneffects.effects.into_iter() {
        let effect_name = jsoneffect.effect;
        let effect = match effect_name.as_str() {
            "allpass" => Box::new(allpass::AllPass) as Box<dyn Effect + Send + Sync>,
            "amplfo" => Box::new(amplfo::AmpLFO::new(fs, jsoneffect.params[0]))
                as Box<dyn Effect + Send + Sync>,
            "echo" => {
                let decay = jsoneffect.params[0];
                let num_echoes = jsoneffect.params[1] as u8;
                let decay_type = if jsoneffect.params[2] == 0. {
                    DecayType::Linear
                } else {
                    DecayType::Exponential
                };
                let start_coef = jsoneffect.params[3];
                Box::new(echo::Echo::new(
                    fs, decay, num_echoes, decay_type, start_coef,
                ))
            }
            "highpass" => Box::new(highpass::IIRHighPass::new(fs, jsoneffect.params[0])),
            "lowpass" => Box::new(lowpass::IIRLowPass::new(fs, jsoneffect.params[0])),
            s => {
                panic!("Error: {} Not A Valid Effect Name", s);
            }
        };
        // add the effect to the list of effects
        resp.push_back(effect);
    }

    // zero out the file so we don't re-parse it
    file.set_len(0).expect("Failed to overwrite ");

    // return our list of effects
    Some(resp)
}
