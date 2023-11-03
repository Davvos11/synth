use nih_plug::prelude::*;
use crate::params::Enable;
use crate::process::note::WaveKind;

#[derive(Params)]
pub struct OscillatorParams {
    #[id = "wave_kind"]
    pub wave_kind: EnumParam<WaveKind>,

    #[id = "pwm"]
    pub pulse_width: FloatParam,

    #[id = "enabled"]
    pub enabled: BoolParam,

    #[id = "volume"]
    pub volume: FloatParam,

    #[id = "transpose"]
    pub transpose: IntParam,

    #[id = "detune"]
    pub detune: FloatParam,
}

impl OscillatorParams {
    pub fn new(index: usize) -> Self {
        Self {
            wave_kind: EnumParam::new(format!("OSC{index} Wave"), WaveKind::Triangle),

            pulse_width: FloatParam::new(
                format!("PW {index}"),
                0.5,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0,
                },
            ).with_smoother(SmoothingStyle::Linear(1.0))
                .with_step_size(0.01),

            enabled: BoolParam::new(format!("OSC{index} Enabled"), index == 0),

            volume: FloatParam::new(
                format!("OSC{index} Volume"),
                -0.01,
                FloatRange::Skewed {
                    min: util::MINUS_INFINITY_DB,
                    max: -0.01,
                    factor: FloatRange::skew_factor(1.0),
                },
            ).with_smoother(SmoothingStyle::Logarithmic(3.0))
                .with_step_size(0.01)
                .with_unit(" dB"),

            transpose: IntParam::new(
                format!("OSC{index} Transpose"),
                0,
                IntRange::Linear {
                    min: -12,
                    max: 12,
                },
            ).with_smoother(SmoothingStyle::Linear(3.0)),

            detune: FloatParam::new(
                format!("OSC{index} Detune"),
                0.0,
                FloatRange::Linear {
                    min: -100.0,
                    max: 100.0,
                }
            ).with_step_size(0.1)
                .with_unit(" cents"),
        }
    }
}

impl Default for OscillatorParams {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Enable for OscillatorParams {
    fn enabled(&self) -> &BoolParam {
        &self.enabled
    }
}