use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use nih_plug::prelude::*;
use crate::cache::ParamCache;
use crate::fixed_map::FixedMap;
use crate::note::{Adsr, Note, Wave, WaveProperties};
use crate::params::get_oscillator_array;
use crate::{OSCILLATOR_AMOUNT, Synth};

pub struct NoteStorage {
    notes: FixedMap<u8, [Note; OSCILLATOR_AMOUNT]>,
    released_notes: Vec<Note>,

    wave_properties: [Arc<Mutex<WaveProperties>>; OSCILLATOR_AMOUNT],
    adsr: Arc<Mutex<Adsr>>,
    oscillator_enabled: [Arc<AtomicBool>; OSCILLATOR_AMOUNT],

    oscillator_range: [usize; OSCILLATOR_AMOUNT],
}

impl NoteStorage {
    pub fn new() -> Self {
        let oscillator_range = get_oscillator_array();

        Self {
            notes: FixedMap::new(64),
            released_notes: Vec::with_capacity(64 * OSCILLATOR_AMOUNT),
            wave_properties: oscillator_range.map(|_| Arc::new(Mutex::new(WaveProperties::default()))),
            adsr: Arc::new(Mutex::new(Adsr::default())),
            oscillator_enabled: oscillator_range.map(|_| Arc::new(AtomicBool::new(false))),
            oscillator_range
        }
    }


    pub fn process_midi(&mut self,
                        event: PluginNoteEvent<Synth>,
                        sample_rate: f32,
    ) {
        match event {
            NoteEvent::NoteOn { note, velocity, .. } => {
                // Create new waves (for each oscillator) for this note
                let new_notes =
                    self.oscillator_range.map(|i| {
                        Note::new(
                            Wave::new(util::midi_note_to_freq(note),
                                      self.wave_properties[i].clone(),
                                      sample_rate,
                                self.oscillator_enabled[i].clone(),
                            ),
                            self.adsr.clone(),
                            velocity,
                        )
                    });

                // Add new notes to map
                let old_notes: Option<[Note; OSCILLATOR_AMOUNT]> = self.notes.insert(note, new_notes);
                // If a note was already playing, release it and save to the list
                if let Some(old_notes) = old_notes {
                    self.release_note(old_notes);
                }
            }
            NoteEvent::NoteOff { note, .. } => {
                let notes = self.notes.remove(&note);
                if let Some(notes) = notes {
                    self.release_note(notes);
                }
            }
            // NoteEvent::PolyPressure { note, pressure, .. }  =>
            //     {
            //         ()
            //     }
            _ => (),
        }
    }

    fn release_note(&mut self, notes: [Note; OSCILLATOR_AMOUNT]) {
        for mut note in notes {
            note.release();
            self.released_notes.push(note);
        };
    }

    pub fn remove_finished_notes(&mut self) {
        self.released_notes.retain(|n| !n.finished());
    }

    pub fn get_sample_value(&mut self) -> f32 {
        // Sum held notes
        let mut new_sample: f32 = self.notes.map.values_mut()
            .flat_map(|notes| {
                notes.iter_mut().map(|note| note.get_sample())
            }).sum();
        // Add sum of released notes
        new_sample += self.released_notes.iter_mut()
            .map(|note| note.get_sample()).sum::<f32>();

        new_sample
    }

    pub fn update_adsr(&mut self, adsr: Adsr) {
        *self.adsr.lock().unwrap() = adsr;
    }

    pub fn update(&mut self, params: &ParamCache) {
        for i in 0..OSCILLATOR_AMOUNT {
            *self.wave_properties[i].lock().unwrap() = params.wave_properties[i].clone();
            self.oscillator_enabled[i].store(params.oscillator_enabled[i], Ordering::Relaxed);
        }
    }
}
