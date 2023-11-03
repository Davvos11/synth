use crate::OSCILLATOR_AMOUNT;
use crate::process::note::OscillatorProperties;
use crate::utils::get_oscillator_array;

pub struct ParamCache {
    pub oscillator_properties: [OscillatorProperties; OSCILLATOR_AMOUNT],
}

impl Default for ParamCache {
    fn default() -> Self {
        Self {
            oscillator_properties: get_oscillator_array().map(|_| OscillatorProperties::default()),
        }
    }
}