#[derive(Clone, Copy)]
pub struct Adsr {
    attack: f32,
    delay: f32,
    sustain: f32,
    release: f32,
}

impl Adsr {
    pub fn new(attack: f32, delay: f32, sustain: f32, release: f32) -> Self {
        Self { attack, delay, sustain, release }
    }
}

pub struct Envelope {
    adsr: Adsr,
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
    pub fn new(adsr: Adsr, sample_rate: f32) -> Self {
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
        // Calculate value based on envelope curve
        let gain = match self.stage {
            Stage::Held => {
                if self.time < self.adsr.attack {
                    // Attack phase
                    self.time / self.adsr.attack
                } else if self.time < self.adsr.attack + self.adsr.delay {
                    // Attack -> sustain phase
                    1.0 -
                        (self.time - self.adsr.attack) / self.adsr.delay *
                            (1.0 - self.adsr.sustain)
                } else {
                    self.adsr.sustain
                }
            }
            Stage::Released {released_at, volume_before} => {
                if self.time <= released_at + self.adsr.release {
                    volume_before - (self.time - released_at) /
                        (self.adsr.release / volume_before)
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
    
    pub fn set_adsr(&mut self, adsr: Adsr) {
        self.adsr = adsr;
    }
}