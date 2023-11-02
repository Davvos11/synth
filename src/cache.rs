use crate::note::WaveProperties;
use crate::params::{get_oscillator_array, OSCILLATOR_AMOUNT};

pub struct ParamCache {
    pub(crate) wave_properties: [WaveProperties; OSCILLATOR_AMOUNT],
}

impl Default for ParamCache {
    fn default() -> Self {
        Self {
            wave_properties: get_oscillator_array().map(|_| WaveProperties::default()),
        }
    }
}