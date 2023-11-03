use nih_plug::prelude::*;
use crate::params::Enable;

#[derive(Params)]
pub struct EnvelopeParams {
    #[id = "enabled"]
    pub enabled: BoolParam,

    #[id = "attack"]
    pub attack: FloatParam,
    #[id = "decay"]
    pub decay: FloatParam,
    #[id = "sustain"]
    pub sustain: FloatParam,
    #[id = "release"]
    pub release: FloatParam,
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