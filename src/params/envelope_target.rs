use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::OSCILLATOR_AMOUNT;

#[derive(Serialize, Deserialize)]
pub struct EnvelopeTargets {
    pub targets: Vec<(Target, f32)>,
}

impl EnvelopeTargets {
    pub fn new() -> Self {
        Self {
            targets: Vec::with_capacity(64),
        }
    }

    pub fn with_target(target: Target) -> Self {
        let mut new = Self::new();
        new.targets.push((target, 1.0));
        new
    }
}

impl Default for EnvelopeTargets {
    fn default() -> Self {
        Self::with_target(Target::None)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Target {
    None,
    AllOscillators,
    Oscillator(usize),
    Parameter(usize)
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::None => write!(f, "-"),
            Target::AllOscillators => write!(f, "All oscillators"),
            Target::Oscillator(i) => write!(f, "Oscillator {i}"),
            Target::Parameter(i) => write!(f, "Parameter {i}"),
        }
    }
}

pub fn get_possible_targets() -> Vec<Target> {
    let mut result = vec![Target::None, Target::AllOscillators];
    for i in 0..OSCILLATOR_AMOUNT {
        result.push(Target::Oscillator(i));
    }
    // TODO parameters

    result
}