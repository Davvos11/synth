use std::f32::consts;
use std::sync::{Arc, Mutex};
use enum_iterator::Sequence;
use nih_plug::prelude::Enum;
use nih_plug::util;
use crate::process::envelope::{Envelope, EnvelopeProperties};


pub struct Note {
    oscillator: Oscillator,
    envelope: Envelope,
    finished: bool,
    velocity: f32,
}

impl Note {
    pub fn new(oscillator: Oscillator, properties: Arc<Mutex<EnvelopeProperties>>, velocity: f32) -> Self {
        let sample_rate = oscillator.sample_rate;

        Self {
            oscillator,
            envelope: Envelope::new(properties, sample_rate),
            finished: false,
            velocity,
        }
    }

    pub fn get_sample(&mut self) -> f32 {
        // Get wave value
        let wave = self.oscillator.get_sample();
        // Get envelope factor
        let (env, finished) = self.envelope.get_gain();

        self.finished = finished;
        wave * env * self.velocity
    }

    pub fn release(&mut self) {
        self.envelope.release()
    }

    pub fn finished(&self) -> bool {
        self.finished
    }
}

#[derive(nih_plug::prelude::Enum, PartialEq, Clone, Copy, Sequence)]
pub enum WaveKind {
    #[id = "sine"]
    Sine,
    #[id = "triangle"]
    Triangle,
    #[id = "saw"]
    Saw,
    #[id = "square"]
    Square,
}

#[derive(Clone)]
pub struct OscillatorProperties {
    kind: WaveKind,
    pulse_width: f32,
    volume: f32,
    enabled: bool,
    transpose: i32,
    detune: f32,
}

impl OscillatorProperties {
    pub fn new(kind: WaveKind, pulse_width: f32, volume: f32, enabled: bool,
               transpose: i32, detune: f32,
    ) -> Self {
        Self {
            kind,
            pulse_width,
            volume,
            enabled,
            transpose,
            detune,
        }
    }
}

impl Default for OscillatorProperties {
    fn default() -> Self {
        Self {
            kind: WaveKind::Sine,
            pulse_width: 0.5,
            volume: 1.0,
            enabled: true,
            transpose: 0,
            detune: 0.0,
        }
    }
}

pub struct Oscillator {
    phase: f32,
    midi_note: u8,
    sample_rate: f32,
    properties: Arc<Mutex<OscillatorProperties>>,
}

impl Oscillator {
    pub fn new(midi_note: u8, properties: Arc<Mutex<OscillatorProperties>>, sample_rate: f32) -> Self {
        Self {
            midi_note,
            phase: 0.0,
            sample_rate,
            properties,
        }
    }

    fn get_sample(&mut self) -> f32 {
        // Get the oscillator properties
        let properties = self.properties.lock().expect("Failed to acquire oscillator_properties lock");

        if !properties.enabled {
            return 0.0;
        }

        // Calculate the frequency
        let frequency =
            util::f32_midi_note_to_freq(
                self.midi_note as f32 +
                    properties.transpose as f32 +
                    (properties.detune / 100.0)
            );
        // Calculate the next step of the wave and phase
        let phase_delta = frequency / self.sample_rate;


        let wave = match properties.kind {
            WaveKind::Sine => {
                (self.phase * consts::TAU).sin()
            }
            WaveKind::Triangle => {
                2.0 * (2.0 * (self.phase - (self.phase + 0.5).floor())).abs() - 1.0
            }
            WaveKind::Saw => {
                2.0 * (self.phase - (self.phase + 0.5).floor())
            }
            WaveKind::Square => {
                if self.phase < properties.pulse_width { 1.0 } else { -1.0 }
            }
        };

        // Update the phase (wrap around if needed)
        self.phase += phase_delta;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        // Return the wave value, at the set volume
        wave * properties.volume
    }
}