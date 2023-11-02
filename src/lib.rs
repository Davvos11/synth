use std::sync::{Arc, Mutex};
use nih_plug::prelude::*;
use triple_buffer::TripleBuffer;
use crate::cache::ParamCache;
use crate::note::{Adsr, WaveProperties};
use crate::params::SynthParams;
use crate::process::notes::NoteStorage;
use crate::process::visual_data::{SynthData, VisualData};

mod gui;
mod note;
mod fixed_map;
mod params;
mod process;
mod cache;

/// The time it takes for the peak meter to decay by 12 dB after switching to complete silence.
const PEAK_METER_DECAY_MS: f64 = 150.0;

pub struct Synth {
    params: Arc<SynthParams>,
    sample_rate: f32,
    notes: NoteStorage,
    data: SynthData,
    visual_data: Arc<Mutex<triple_buffer::Output<VisualData>>>,
    param_cache: ParamCache,
}


impl Default for Synth {
    fn default() -> Self {
        let (synth_data_input, synth_data_output) = TripleBuffer::default().split();

        Self {
            params: Arc::new(SynthParams::default()),
            sample_rate: 1.0,
            notes: NoteStorage::new(),
            data: SynthData::new(synth_data_input),
            visual_data: Arc::new(Mutex::new(synth_data_output)),
            param_cache: ParamCache::default(),
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
            self.visual_data.clone(),
        )
    }

    fn initialize(&mut self, _audio_io_layout: &AudioIOLayout, buffer_config: &BufferConfig, _context: &mut impl InitContext<Self>) -> bool {
        self.sample_rate = buffer_config.sample_rate;

        // After `PEAK_METER_DECAY_MS` milliseconds of pure silence, the peak meter's value should
        // have dropped by 12 dB
        self.data.set_peak_meter_decay_weight(0.25f64
            .powf((buffer_config.sample_rate as f64 * PEAK_METER_DECAY_MS / 1000.0).recip())
            as f32);

        // Load initial param data
        self.get_wave_properties();
        true
    }

    fn process(&mut self, buffer: &mut Buffer, _aux: &mut AuxiliaryBuffers, context: &mut impl ProcessContext<Self>) -> ProcessStatus {
        for (_sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            // Get ui parameters
            let volume = self.params.volume.smoothed.next();
            let adsr = self.get_adsr();
            self.get_wave_properties();

            // Update parameters of notes that are playing
            self.notes.update_adsr(adsr);
            self.notes.update_wave_properties(&self.param_cache.wave_properties);

            // Process midi (modifies `self.notes` and `self.released_notes`)
            let mut next_event = context.next_event();

            // Loop over midi events
            while let Some(event) = next_event {
                // Break if we encounter midi events for the next sample buffer
                // Removed because it introduced bugs
                // TODO find out whether this has performance effects
                // if event.timing() > sample_id as u32 {
                //     break;
                // }

                self.notes.process_midi(
                    event,
                    self.sample_rate,
                );

                next_event = context.next_event();
            }

            // Calculate output value, by summing all waves
            let new_sample = self.notes.get_sample_value() * util::db_to_gain_fast(volume);

            for sample in channel_samples {
                *sample = new_sample;
            }

            // Remove finished notes
            self.notes.remove_finished_notes();

            // Calculate volume meter
            if self.params.editor_state.is_open() {
                self.data.set_visual_data(new_sample);
            }
        }

        ProcessStatus::Normal
    }
}

impl Synth {
    fn get_adsr(&self) -> Adsr {
        Adsr::new(
            self.params.attack.smoothed.next(),
            self.params.decay.smoothed.next(),
            util::db_to_gain_fast(self.params.sustain.smoothed.next()),
            self.params.release.smoothed.next(),
        )
    }

    fn get_wave_properties(&mut self) {
        self.params.oscillator_params.iter().enumerate().for_each(|(i, params)| {
            self.param_cache.wave_properties[i] = WaveProperties::new(
                params.wave_kind.value(),
                params.pulse_width.value(),
            );
        });
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