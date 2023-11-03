use std::sync::{Arc, Mutex};

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
        Self {attack: 0.01, decay: 0.0, sustain: 1.0, release: 0.01}
    }
}

pub struct Envelope {
    adsr: Arc<Mutex<Adsr>>,
    delta: f32,
    time: f32,
    last_volume: f32,
    stage: Stage,
}

#[derive(PartialEq)]
enum Stage {
    Held,
    Released {
        released_at: f32,
        volume_before: f32,
    },
    Finished,
}

impl Envelope {
    pub fn new(adsr: Arc<Mutex<Adsr>>, sample_rate: f32) -> Self {
        Self {
            adsr,
            delta: 1.0 / sample_rate,
            time: 0.0,
            stage: Stage::Held,
            last_volume: 0.0,
        }
    }

    pub fn release(&mut self) {
        self.stage = Stage::Released { released_at: self.time, volume_before: self.last_volume };
    }

    pub fn get_gain(&mut self) -> (f32, bool) {
        // Get ADSR value
        let adsr = *self.adsr.lock().expect("Failed to acquire ADSR lock");
        
        // Calculate value based on envelope curve
        let gain = match self.stage {
            Stage::Held => {
                if self.time < adsr.attack {
                    // Attack phase
                    self.time / adsr.attack
                } else if self.time < adsr.attack + adsr.decay {
                    // Attack -> sustain phase
                    1.0 -
                        (self.time - adsr.attack) / adsr.decay *
                            (1.0 - adsr.sustain)
                } else {
                    adsr.sustain
                }
            }
            Stage::Released {released_at, volume_before} => {
                if self.time <= released_at + adsr.release {
                    volume_before - (self.time - released_at) /
                        (adsr.release / volume_before)
                } else {
                    self.stage = Stage::Finished;
                    0.0
                }
            }
            Stage::Finished => {
                0.0
            }
        };

        // Update internal time
        self.time += self.delta;

        let finished = self.stage == Stage::Finished;
        self.last_volume = gain;
        (gain, finished)
    }
    
}