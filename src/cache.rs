use crate::note::WaveProperties;
use crate::OSCILLATOR_AMOUNT;
use crate::params::get_oscillator_array;

pub struct ParamCache {
    pub(crate) wave_properties: [WaveProperties; OSCILLATOR_AMOUNT],
    pub(crate) oscillator_enabled: [bool; OSCILLATOR_AMOUNT],
}

impl Default for ParamCache {
    fn default() -> Self {
        Self {
            wave_properties: get_oscillator_array().map(|_| WaveProperties::default()),
            oscillator_enabled: get_oscillator_array().map(|_| false),
        }
    }
}