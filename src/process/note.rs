use std::f32::consts;
use std::sync::{Arc, Mutex};
use enum_iterator::Sequence;
use nih_plug::prelude::Enum;
use nih_plug::util;
use crate::ENVELOPE_AMOUNT;
use crate::params::envelope_target::Target;
use crate::process::envelope::{Adsr, EnvelopeProperties, Stage};

pub struct Note {
    midi_note: u8,
    velocity: f32,
    sample_rate: f32,
    oscillator_id: usize,
    time: f32,
    phase: f32,

    oscillator_properties: Arc<Mutex<OscillatorProperties>>,
    envelope_properties: Arc<Mutex<[EnvelopeProperties; ENVELOPE_AMOUNT]>>,
    pub stage: Stage,
    pub last_env_gain: f32,
}

impl Note {
    pub fn new(midi_note: u8, velocity: f32, sample_rate: f32,
               oscillator_id: usize, oscillator_properties: Arc<Mutex<OscillatorProperties>>,
               envelope_properties: Arc<Mutex<[EnvelopeProperties; ENVELOPE_AMOUNT]>>) -> Self {
        Self {
            midi_note,
            velocity,
            sample_rate,
            oscillator_id,
            oscillator_properties,
            envelope_properties,
            time: 0.0,
            phase: 0.0,
            stage: Stage::Held,
            last_env_gain: 0.0,
        }
    }

    pub fn release(&mut self) {
        self.stage = Stage::Released {released_at: self.time, gain_before: self.last_env_gain }
    }

    pub fn get_sample(&mut self) -> f32 {
        let sample = self.get_wave_sample();

        // Get envelope gain
        let env_gain = self.get_envelope_gain();

        sample * env_gain * self.velocity
    }

    pub fn is_finished(&self) -> bool {
        self.stage == Stage::Finished
    }

    fn get_wave_sample(&mut self) -> f32 {
        // Get the oscillator properties
        let osc_properties = self.oscillator_properties.lock()
            .expect("Failed to acquire oscillator_properties lock");

        if !osc_properties.enabled { return 0.0; }


        // Calculate the frequency
        let frequency =
            util::f32_midi_note_to_freq(
                self.midi_note as f32
                    + osc_properties.transpose as f32
                    + (osc_properties.detune / 100.0)
            );

        // Get wave value
        let sample = get_wave_sample(osc_properties.kind, self.phase, osc_properties.pulse_width);

        // Update phase
        self.phase += frequency / self.sample_rate;
        self.phase %= 1.0;

        sample * osc_properties.volume
    }

    // TODO multiple envelopes
    fn get_envelope_gain(&mut self) -> f32 {
        if let Some(envelope) = self.get_envelope() {
            let env_gain = get_env_gain(envelope.adsr, &self.stage, self.time);
            // Update stage
            if env_gain.finished { self.stage = Stage::Finished; }
            self.last_env_gain = env_gain.gain;
            // Update time
            self.time += 1.0 / self.sample_rate;
            env_gain.gain
        } else {
            1.0
        }
    }

    fn get_envelope(&self) -> Option<EnvelopeProperties> {
        let envelope_properties = self.envelope_properties.lock()
            .expect("Failed to aquire envelope_properties lock");
        for envelope in envelope_properties.iter() {
            if envelope.has_target(Target::AllOscillators) ||
                envelope.has_target(Target::Oscillator(self.oscillator_id)) {
                return Some(envelope.clone());
            }
        }
        None
    }
}

fn get_wave_sample(wave: WaveKind, phase: f32, pulse_width: f32) -> f32 {
    match wave {
        WaveKind::Sine => {
            (phase * consts::TAU).sin()
        }
        WaveKind::Triangle => {
            2.0 * (2.0 * (phase - (phase + 0.5).floor())).abs() - 1.0
        }
        WaveKind::Saw => {
            2.0 * (phase - (phase + 0.5).floor())
        }
        WaveKind::Square => {
            if phase < pulse_width { 1.0 } else { -1.0 }
        }
    }
}

fn get_env_gain(adsr: Adsr, stage: &Stage, time: f32) -> Gain {
    match stage {
        Stage::Held => {
            if time < adsr.attack() {
                // Attack phase
                Gain::new(time / adsr.attack())
            } else if time < adsr.attack() + adsr.decay() {
                // Attack -> sustain phase
                Gain::new(
                1.0 - (time - adsr.attack()) / adsr.decay() * (1.0 - adsr.sustain())
                )
            } else {
                Gain::new(adsr.sustain())
            }
        }
        Stage::Released { released_at, gain_before: volume_before } => {
            if time <= released_at + adsr.release() {
                Gain::new(
                volume_before - (time - released_at) /
                    (adsr.release() / volume_before)
                )
            } else {
                Gain::finished()
            }
        }
        Stage::Finished => {
            Gain::finished()
        }
    }
}

struct Gain {
    gain: f32,
    finished: bool
}

impl Gain {
    pub fn new(gain: f32) -> Self {
        Self { gain, finished: false }
    }

    pub fn finished() -> Self {
        Self { gain: 0.0, finished: true }
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