use nih_plug::prelude::*;
use crate::fixed_map::FixedMap;
use crate::note::{Adsr, Note, Wave, WaveKind};
use crate::Synth;

pub struct NoteStorage {
    notes: FixedMap<u8, Note>,
    released_notes: Vec<Note>,
}

impl NoteStorage {
    pub fn new() -> Self {
        Self {
            notes: FixedMap::new(64),
            released_notes: Vec::with_capacity(64),
        }
    }


    pub fn process_midi(&mut self,
                        event: PluginNoteEvent<Synth>,
                        sample_rate: f32,
                        adsr: Adsr,
    ) {
        match event {
            NoteEvent::NoteOn { note, velocity, .. } => {
                // Create new sine wave for this note
                let new_note = Note::new(
                    Wave::new(util::midi_note_to_freq(note),
                              WaveKind::Square,
                              sample_rate
                    ),
                    adsr,
                    velocity,
                );
                // Add new note to map
                let old_note = self.notes.insert(note, new_note);
                // If a note was already playing, release it and save to the list
                if let Some(old_note) = old_note {
                    self.release_note(old_note);
                }
            }
            NoteEvent::NoteOff { note, .. } => {
                let note = self.notes.remove(&note);
                if let Some(note) = note {
                    self.release_note(note);
                }
            }
            // NoteEvent::PolyPressure { note, pressure, .. }  =>
            //     {
            //         ()
            //     }
            _ => (),
        }
    }

    fn release_note(&mut self, mut note: Note) {
        note.release();
        self.released_notes.push(note);
    }

    pub fn remove_finished_notes(&mut self) {
        self.released_notes.retain(|n| !n.finished());
    }

    pub fn get_sample_value(&mut self) -> f32 {
        let mut new_sample = self.notes.map.values_mut()
            .map(|n| n.get_sample()).sum();
        new_sample += self.released_notes.iter_mut()
            .map(|n| n.get_sample()).sum::<f32>();

        new_sample
    }
}