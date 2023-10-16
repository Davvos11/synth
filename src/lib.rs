use std::f32::consts;
use std::sync::Arc;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

mod gui;

pub struct Synth {
    params: Arc<SynthParams>,
    sample_rate: f32,
    phase: f32,
}

impl Default for Synth {
    fn default() -> Self {
        Self {
            params: Arc::new(SynthParams::default()),
            sample_rate: 1.0,
            phase: 0.0,
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

    #[id = "freq"]
    pub frequency: FloatParam,
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
            frequency: FloatParam::new(
                "Frequency",
                420.0,
                FloatRange::Skewed {
                    min: 20.0,
                    max: 20_000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            ).with_smoother(SmoothingStyle::Linear(10.0))
                .with_value_to_string(formatters::v2s_f32_hz_then_khz(0))
                .with_string_to_value(formatters::s2v_f32_hz_then_khz()),
        }
    }
}

impl Synth {
    fn calculate_sine(&mut self, frequency: f32) -> f32 {
        // Calculate the next step of the sine and phase
        let phase_delta = frequency / self.sample_rate;
        let sine = (self.phase * consts::TAU).sin();

        // Update the phase (wrap around if needed)
        self.phase += phase_delta;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        // Return the sine value
        sine
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
        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            // Get gain and frequency (smoothed values)
            let gain = self.params.gain.smoothed.next();
            let frequency = self.params.frequency.smoothed.next();
            // Calculate output value
            let sine = self.calculate_sine(frequency);

            // Set samples
            for sample in channel_samples {
                *sample = sine * util::db_to_gain_fast(gain);
            }
        }

        ProcessStatus::KeepAlive
    }
}