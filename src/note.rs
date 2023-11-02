use std::f32::consts;
use std::sync::{Arc, Mutex};
use enum_iterator::Sequence;
use nih_plug::prelude::Enum;
pub use crate::note::envelope::Adsr;
use crate::note::envelope::Envelope;

mod envelope;


pub struct Note {
    wave: Wave,
    envelope: Envelope,
    finished: bool,
    velocity: f32,
}

impl Note {
    pub fn new(wave: Wave, adsr: Arc<Mutex<Adsr>>, velocity: f32) -> Self {
        let sample_rate = wave.sample_rate;

        Self {
            wave,
            envelope: Envelope::new(adsr, sample_rate),
            finished: false,
            velocity,
        }
    }

    pub fn get_sample(&mut self) -> f32 {
        // Get wave value
        let wave = self.wave.get_sample();
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
pub struct WaveProperties {
    kind: WaveKind,
    pulse_width: f32,
}

impl WaveProperties {
    pub fn new(kind: WaveKind, pulse_width: f32) -> Self {
        Self { kind, pulse_width }
    }
}

impl Default for WaveProperties {
    fn default() -> Self {
        Self {
            kind: WaveKind::Sine,
            pulse_width: 0.5,
        }
    }
}

pub struct Wave {
    phase: f32,
    frequency: f32,
    sample_rate: f32,
    properties: Arc<Mutex<WaveProperties>>,
}

impl Wave {
    pub fn new(frequency: f32, properties: Arc<Mutex<WaveProperties>>, sample_rate: f32) -> Self {
        Self {
            frequency,
            phase: 0.0,
            sample_rate,
            properties,
        }
    }

    fn get_sample(&mut self) -> f32 {
        // Calculate the next step of the wave and phase
        let phase_delta = self.frequency / self.sample_rate;

        // Get the wave properties
        let properties = self.properties.lock().expect("Failed to acquire wave_kind lock");


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

        // Return the sine value
        wave
    }
}