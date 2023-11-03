use std::sync::Arc;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use crate::{gui, OSCILLATOR_AMOUNT};
use crate::note::WaveKind;

pub fn get_oscillator_array() -> [usize; OSCILLATOR_AMOUNT] {
    (0..OSCILLATOR_AMOUNT).map(|x| x as usize).collect::<Vec<usize>>().try_into().unwrap()
}

#[derive(Params)]
pub struct SynthParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "volume"]
    pub volume: FloatParam,

    #[id = "attack"]
    pub attack: FloatParam,
    #[id = "decay"]
    pub decay: FloatParam,
    #[id = "sustain"]
    pub sustain: FloatParam,
    #[id = "release"]
    pub release: FloatParam,

    #[nested(array, group = "Oscillator Parameters")]
    pub oscillator_params: [OscillatorParams; OSCILLATOR_AMOUNT],
}

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

impl Default for SynthParams {
    fn default() -> Self {
        Self {
            editor_state: gui::default_state(),

            volume: FloatParam::new(
                "Volume",
                -10.0,
                FloatRange::Skewed {
                    min: util::MINUS_INFINITY_DB,
                    max: -0.01,
                    factor: FloatRange::skew_factor(1.0),
                },
            ).with_smoother(SmoothingStyle::Logarithmic(3.0))
                .with_step_size(0.01)
                .with_unit(" dB"),

            attack: FloatParam::new(
                "Attack",
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
                "Decay",
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
                "Sustain",
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
                "Release",
                0.2,
                FloatRange::Skewed {
                    min: 0.01,
                    max: 10.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            ).with_smoother(SmoothingStyle::Linear(3.0))
                .with_step_size(0.01)
                .with_unit(" sec"),

            oscillator_params: get_oscillator_array().map(|i| {
                OscillatorParams::new(i)
            }),
        }
    }
}

impl OscillatorParams {
    fn new(index: usize) -> Self {
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