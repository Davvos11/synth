use std::sync::Arc;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use crate::{ENVELOPE_AMOUNT, gui, OSCILLATOR_AMOUNT};
use crate::process::note::WaveKind;
use crate::utils::{get_envelope_array, get_oscillator_array};

#[derive(Params)]
pub struct SynthParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,

    #[id = "volume"]
    pub volume: FloatParam,

    #[nested(array, group = "Oscillator Parameters")]
    pub oscillator_params: [OscillatorParams; OSCILLATOR_AMOUNT],

    #[nested(array, group = "Envelope Parameters")]
    pub envelope_params: [EnvelopeParams; ENVELOPE_AMOUNT],
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

            oscillator_params: get_oscillator_array().map(|i| {
                OscillatorParams::new(i)
            }),

            envelope_params: get_envelope_array().map(|i| {
                EnvelopeParams::new(i)
            })
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

impl Enable for OscillatorParams {
    fn enabled(&self) -> &BoolParam {
        &self.enabled
    }
}

impl EnvelopeParams {
    fn new(index: usize) -> Self {
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

pub trait Enable {
    fn enabled(&self) -> &BoolParam;
}