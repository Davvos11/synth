use std::collections::HashMap;
use std::sync::Arc;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use crate::note::Sine;

mod gui;
mod note;

pub struct Synth {
    params: Arc<SynthParams>,
    sample_rate: f32,
    notes: HashMap<u8, Sine>,
}

impl Default for Synth {
    fn default() -> Self {
        Self {
            params: Arc::new(SynthParams::default()),
            sample_rate: 1.0,
            // We have to add a max capacity because relocating objects is not
            // supported in nih_plug.
            // TODO add check for when this capacity is reached
            notes: HashMap::with_capacity(64),
        }
    }
}

#[derive(Params)]
struct SynthParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,

    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for SynthParams {
    fn default() -> Self {
        Self {
            editor_state: gui::default_state(),

            gain: FloatParam::new(
                "Gain",
                -10.0,
                FloatRange::Linear {
                    min: -30.0,
                    max: 0.0,
                },
            ).with_smoother(SmoothingStyle::Linear(3.0))
                .with_step_size(0.01)
                .with_unit(" dB"),
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

    fn editor(&mut self, async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        gui::create(
            self.params.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(&mut self, audio_io_layout: &AudioIOLayout, buffer_config: &BufferConfig, context: &mut impl InitContext<Self>) -> bool {
        self.sample_rate = buffer_config.sample_rate;

        true
    }

    fn process(&mut self, buffer: &mut Buffer, aux: &mut AuxiliaryBuffers, context: &mut impl ProcessContext<Self>) -> ProcessStatus {
        // Get midi event
        let mut next_event = context.next_event();

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            // Get gain value
            let gain = self.params.gain.smoothed.next();

            // Loop over midi events
            while let Some(event) = next_event {
                // Break if we encounter midi events for the next sample buffer
                if event.timing() > sample_id as u32 {
                    break;
                }

                match event {
                    NoteEvent::NoteOn { note, velocity, .. } => {
                        // Create new sine wave for this note
                        let wave = Sine::new(
                            util::midi_note_to_freq(note),
                            velocity,
                            self.sample_rate);
                        self.notes.insert(note, wave);
                        // TODO, stop playing the old value
                    }
                    NoteEvent::NoteOff { note, .. } => {
                        self.notes.remove(&note);
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
            let new_sample: f32 = self.notes.values_mut()
                .map(|e| e.get_sample())
                .sum();
            let new_sample = new_sample * util::db_to_gain_fast(gain);

            for sample in channel_samples {
                *sample = new_sample;
            }
        }

        ProcessStatus::KeepAlive
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