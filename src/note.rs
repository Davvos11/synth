use std::f32::consts;

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

    pub fn get_sample(&mut self) -> f32 {
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