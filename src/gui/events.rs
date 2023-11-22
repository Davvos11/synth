use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use nih_plug::prelude::ParamSetter;
use crate::OSCILLATOR_AMOUNT;
use crate::params::Enable;
use crate::params::envelope_target::Target;

pub enum ControlEvent {
    AddOscillator,
    RemoveOscillator,
    AddEnvelope,
    RemoveEnvelope,
    SetEnvelopeTarget(usize, usize, Target),
    SetEnvelopeTargetDepth(usize, usize, f32),
}

pub fn add_item<T>(params: &[T; OSCILLATOR_AMOUNT],
                setter: ParamSetter,
                is_max: Arc<AtomicBool>,
)   where T: Enable
{
    let mut all_enabled = true;
    let mut set = false;
    // Loop over oscillators, setting the first that is disabled
    // and checking if all have been enabled or not
    for param in params {
        // Find first oscillator that is disabled
        if !param.enabled().value() {
            if set {
                all_enabled = false;
                break;
            }
            // Enable
            setter.begin_set_parameter(param.enabled());
            setter.set_parameter(param.enabled(), true);
            setter.end_set_parameter(param.enabled());
            // Go to next loop
            set = true;
            continue;
        }
    }

    if all_enabled {
        is_max.store(true, Ordering::Relaxed);
    }
}