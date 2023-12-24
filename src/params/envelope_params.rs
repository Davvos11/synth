use std::sync::{Arc, Mutex};
use nih_plug::prelude::*;
use crate::params::Enable;
use crate::params::envelope_target::{EnvelopeTargets, Target};

#[derive(Params)]
pub struct EnvelopeParams {
    #[id = "on"]
    pub enabled: BoolParam,

    #[id = "a"]
    pub attack: FloatParam,
    #[id = "d"]
    pub decay: FloatParam,
    #[id = "s"]
    pub sustain: FloatParam,
    #[id = "r"]
    pub release: FloatParam,

    // TODO test if this properly persists in a plugin host
    #[persist = "targets"]
    pub targets: Arc<Mutex<EnvelopeTargets>>,
}

impl EnvelopeParams {
    pub fn new(index: usize) -> Self {
        Self {
            enabled: BoolParam::new(
                format!("ENV{index} Enabled"),
                index == 0,
            ),

            attack: FloatParam::new(
                format!("ENV{index} Attack"),
                0.01,
                FloatRange::Skewed {
                    min: 0.01,
                    max: 10.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            ).with_smoother(SmoothingStyle::Linear(3.0))
                .with_step_size(0.01)
                .with_unit(" sec"),

            decay: FloatParam::new(
                format!("ENV{index} Decay"),
                0.2,
                FloatRange::Skewed {
                    min: 0.01,
                    max: 10.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            ).with_smoother(SmoothingStyle::Linear(3.0))
                .with_step_size(0.01)
                .with_unit(" sec"),

            sustain: FloatParam::new(
                format!("ENV{index} Sustain"),
                -10.0,
                FloatRange::Skewed {
                    min: util::MINUS_INFINITY_DB,
                    max: -0.01,
                    factor: FloatRange::skew_factor(1.0),
                },
            ).with_smoother(SmoothingStyle::Logarithmic(3.0))
                .with_step_size(0.01)
                .with_unit(" dB"),

            release: FloatParam::new(
                format!("ENV{index} Release"),
                0.2,
                FloatRange::Skewed {
                    min: 0.01,
                    max: 10.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            ).with_smoother(SmoothingStyle::Linear(3.0))
                .with_step_size(0.01)
                .with_unit(" sec"),

            targets: Arc::new(Mutex::new(
                if index == 0 {
                    EnvelopeTargets::with_target(Target::AllOscillators)
                } else {
                    EnvelopeTargets::default()
                }
            )),
        }
    }
}

impl Default for EnvelopeParams {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Enable for EnvelopeParams {
    fn enabled(&self) -> &BoolParam {
        &self.enabled
    }
}