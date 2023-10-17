use std::sync::Arc;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use crate::fixed_map::FixedMap;
use crate::note::{Adsr, Note, Sine};

mod gui;
mod note;
mod fixed_map;

pub struct Synth {
    params: Arc<SynthParams>,
    sample_rate: f32,
    notes: FixedMap<u8, Note>,
    released_notes: Vec<Note>,
}

impl Default for Synth {
    fn default() -> Self {
        Self {
            params: Arc::new(SynthParams::default()),
            sample_rate: 1.0,
            notes: FixedMap::new(64),
            released_notes: Vec::with_capacity(64),
        }
    }
}

#[derive(Params)]
struct SynthParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    #[id = "volume"]
    pub volume: FloatParam,

    #[id = "attack"]
    pub attack: FloatParam,
    #[id = "decay"]
    pub decay: FloatParam,
    #[id = "sustain"]
    pub sustain: FloatParam,
    #[id = "release"]
    pub release: FloatParam,
}

impl Default for SynthParams {
    fn default() -> Self {
        Self {
            editor_state: gui::default_state(),

            volume: FloatParam::new(
                "Volume",
                -10.0,
                FloatRange::Linear {
                    min: -30.0,
                    max: 0.0,
                },
            ).with_smoother(SmoothingStyle::Linear(3.0))
                .with_step_size(0.01)
                .with_unit(" dB"),

            attack: FloatParam::new(
                "Attack",
                0.01,
                FloatRange::Linear {
                    min: 0.0,
                    max: 10.0,
                }
            ).with_smoother(SmoothingStyle::Linear(3.0))
                .with_step_size(0.01)
                .with_unit("sec"),

            decay: FloatParam::new(
                "Decay",
                0.2,
                FloatRange::Linear {
                    min: 0.0,
                    max: 10.0,
                }
            ).with_smoother(SmoothingStyle::Linear(3.0))
                .with_step_size(0.01)
                .with_unit("sec"),

            sustain: FloatParam::new(
                "Sustain",
                -10.0,
                FloatRange::Linear {
                    min: -30.0,
                    max: 0.0,
                }
            ).with_smoother(SmoothingStyle::Linear(3.0))
                .with_step_size(0.01)
                .with_unit("dB"),

            release: FloatParam::new(
                "Release",
                0.2,
                FloatRange::Linear {
                    min: 0.0,
                    max: 10.0,
                }
            ).with_smoother(SmoothingStyle::Linear(3.0))
                .with_step_size(0.01)
                .with_unit("sec"),
        }
    }
}

impl Plugin for Synth {
    const NAME: &'static str = "Synth";
    const VENDOR: &'static str = "Davvos11";
    const URL: &'static str = "https://dovatvis.nl";
    const EMAIL: &'static str = "vosdavid2@gmail.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            // This is also the default and can be omitted here
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        gui::create(
            self.params.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(&mut self, _audio_io_layout: &AudioIOLayout, buffer_config: &BufferConfig, _context: &mut impl InitContext<Self>) -> bool {
        self.sample_rate = buffer_config.sample_rate;

        true
    }

    fn process(&mut self, buffer: &mut Buffer, _aux: &mut AuxiliaryBuffers, context: &mut impl ProcessContext<Self>) -> ProcessStatus {
        // Get midi event
        let mut next_event = context.next_event();

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            // Get ui parameters
            let volume = self.params.volume.smoothed.next();
            let adsr = self.get_adsr();

            // Loop over midi events
            while let Some(event) = next_event {
                // Break if we encounter midi events for the next sample buffer
                if event.timing() > sample_id as u32 {
                    break;
                }

                match event {
                    NoteEvent::NoteOn { note, velocity, .. } => {
                        // Create new sine wave for this note
                        let new_note = Note::new(
                            Sine::new(
                                util::midi_note_to_freq(note),
                                velocity,
                                self.sample_rate),
                            adsr,
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

                next_event = context.next_event();
            }

            // Calculate output value, by summing all waves
            let mut new_sample = self.notes.map.values_mut()
                .map(|n| n.get_sample()).sum();
            new_sample += self.released_notes.iter_mut()
                .map(|n| n.get_sample()).sum::<f32>();

            // Apply volume
            new_sample *= util::db_to_gain_fast(volume);

            for sample in channel_samples {
                *sample = new_sample;
            }

            // Remove finished notes
            self.released_notes.retain(|n| !n.finished());
        }

        ProcessStatus::KeepAlive
    }
}

impl Synth {
    fn release_note(&mut self, mut note: Note) {
        note.release();
        self.released_notes.push(note);
    }

    fn get_adsr(&self) -> Adsr {
        Adsr::new(
            self.params.attack.smoothed.next(),
            self.params.decay.smoothed.next(),
            util::db_to_gain_fast(self.params.sustain.smoothed.next()),
            self.params.release.smoothed.next(),
        )
    }
}

impl Vst3Plugin for Synth {
    const VST3_CLASS_ID: [u8; 16] = *b"SineMoistestPlug";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Instrument,
        Vst3SubCategory::Synth,
        Vst3SubCategory::Tools,
    ];
}

nih_export_vst3!(Synth);