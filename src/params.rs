use std::sync::Arc;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use crate::{ENVELOPE_AMOUNT, gui, OSCILLATOR_AMOUNT};
use crate::params::envelope_params::EnvelopeParams;
use crate::params::oscillator_params::OscillatorParams;
use crate::utils::{get_envelope_array, get_oscillator_array};

mod envelope_params;
mod oscillator_params;

pub trait Enable {
    fn enabled(&self) -> &BoolParam;
}

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