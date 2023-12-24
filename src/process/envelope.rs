use std::sync::{Arc, Mutex};
use crate::params::envelope_target::{EnvelopeTargets, Target};

#[derive(Clone)]
pub struct EnvelopeProperties {
    pub adsr: Adsr,
    pub targets:  Arc<Mutex<EnvelopeTargets>>,
}

impl EnvelopeProperties {
    pub fn new(adsr: Adsr, targets: Arc<Mutex<EnvelopeTargets>>) -> Self {
        Self { adsr, targets }
    }

    pub fn get_amount_for(&self, target: Target) -> f32 {
        let targets = self.targets.lock()
            .expect("Failed to acquire envelope_targets lock");
        targets.get_amount_for(target)
    }
}

impl Default for EnvelopeProperties {
    fn default() -> Self {
        Self {
            adsr: Adsr::default(),
            targets: Arc::new(Mutex::new(EnvelopeTargets::default())),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Adsr {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
}

impl Adsr {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self { attack, decay, sustain, release }
    }


    pub fn attack(&self) -> f32 {
        self.attack
    }
    pub fn decay(&self) -> f32 {
        self.decay
    }
    pub fn sustain(&self) -> f32 {
        self.sustain
    }
    pub fn release(&self) -> f32 {
        self.release
    }
}

impl Default for Adsr {
    fn default() -> Self {
        Self { attack: 0.01, decay: 0.0, sustain: 1.0, release: 0.01 }
    }
}

#[derive(PartialEq, Clone)]
pub enum Stage {
    Held,
    Released {
        released_at: f32,
        gain_before: f32,
    },
    Finished,
}

