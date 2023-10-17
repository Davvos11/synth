use std::f32::consts;
pub use crate::note::envelope::Adsr;
use crate::note::envelope::Envelope;

mod envelope;


pub struct Note {
    wave: Sine,
    envelope: Envelope,
    finished: bool,
}

impl Note {
    pub fn new(wave: Sine, adsr: Adsr) -> Self {
        let sample_rate = wave.sample_rate;

        Self {
            wave,
            envelope: Envelope::new(adsr, sample_rate),
            finished: false,
        }
    }

    pub fn get_sample(&mut self) -> f32 {
        // Get wave value
        let wave = self.wave.get_sample();
        // Get envelope factor
        let (env, finished) = self.envelope.get_gain();
        
        self.finished = finished;
        wave * env
    }

    pub fn release(&mut self) {
        self.envelope.release()
    }
    
    pub fn finished(&self) -> bool {
        self.finished
    }
}

trait Wave {
    fn get_sample(&mut self) -> f32;

    fn sample_rate(&self) -> f32;
}

pub struct Sine {
    phase: f32,
    gain_factor: f32,
    frequency: f32,
    sample_rate: f32,
}

impl Sine {
    pub fn new(frequency: f32, velocity: f32, sample_rate: f32) -> Self {
        Self {
            gain_factor: velocity,
            frequency,
            phase: 0.0,
            sample_rate,
        }
    }
}

impl Sine {
    fn get_sample(&mut self) -> f32 {
        // Calculate the next step of the sine and phase
        let phase_delta = self.frequency / self.sample_rate;
        let sine = (self.phase * consts::TAU).sin();

        // Update the phase (wrap around if needed)
        self.phase += phase_delta;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        // Return the sine value
        sine * self.gain_factor
    }
}