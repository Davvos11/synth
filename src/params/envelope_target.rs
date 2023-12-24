use std::fmt::{Display, Formatter};
use std::sync::MutexGuard;
use nih_plug_vizia::vizia::prelude::*;
use serde::{Deserialize, Serialize};
use crate::OSCILLATOR_AMOUNT;
use crate::gui::events::ControlEvent;

#[derive(Serialize, Deserialize, Lens, Clone)]
pub struct EnvelopeTargets {
    // TODO use hashmap
    pub targets: Vec<(Target, f32)>,
}

impl EnvelopeTargets {
    pub fn from(from: MutexGuard<Self>) -> Self {
        Self {
            targets: from.targets.to_vec()
        }
    }
    
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

    pub fn add(&mut self) {
        self.targets.push((Target::None, 1.0))
    }

    pub fn remove(&mut self, index: usize) -> (Target, f32) {
        self.targets.remove(index)
    }

    pub fn get_amount_for(&self, target: Target) -> f32 {
        self.targets.iter().filter_map(|(t, d)|{
            if *t == target { Some(d) }
            else { None }
        }).sum()
    }
}

impl Default for EnvelopeTargets {
    fn default() -> Self {
        // TODO set to 0 so it takes up more memory, does that make sense?
        Self::with_target(Target::Oscillator(0))
    }
}

impl Model for EnvelopeTargets {
    #[allow(clippy::single_match)]
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|control_event: &ControlEvent, _meta|
            match control_event {
                ControlEvent::AddEnvelopeTarget => {
                    self.add();
                }
                ControlEvent::RemoveEnvelopeTarget(i) => {
                    self.remove(*i);
                }
                _ => {}
            }
        );
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