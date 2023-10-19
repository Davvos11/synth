use nih_plug::util;

pub struct SynthData {
    data: triple_buffer::Input<VisualData>,
    // Depends on sample rate:
    peak_meter_decay_weight: f32,
}

impl SynthData {
    pub fn new(mut data_input: triple_buffer::Input<VisualData>) -> Self {
        data_input.write(VisualData::default());

        Self {
            data: data_input,
            peak_meter_decay_weight: 1.0,
        }
    }
}

#[derive(Clone)]
pub struct VisualData {
    pub peak_meter: f32,
    pub samples: Vec<f32>,
}

impl Default for VisualData {
    fn default() -> Self {
        Self {
            peak_meter: util::MINUS_INFINITY_DB,
            samples: Vec::with_capacity(10_000),
        }
    }
}

impl SynthData {
    pub fn set_visual_data(&mut self,
                           new_sample: f32,
    ) {
        // Load data input buffer
        let data = self.data.input_buffer();

        // Calculate peak
        let amplitude = new_sample.abs();
        data.peak_meter = if amplitude > data.peak_meter {
            amplitude
        } else {
            data.peak_meter * self.peak_meter_decay_weight
                + amplitude * (1.0 - self.peak_meter_decay_weight)
        };

        // Save new sample to moving buffer
        while data.samples.len() >= data.samples.capacity() {
            data.samples.remove(0);
        }
        data.samples.push(new_sample);

        // Publish input buffer
        self.data.publish();
    }

    pub fn set_peak_meter_decay_weight(&mut self, peak_meter_decay_weight: f32) {
        self.peak_meter_decay_weight = peak_meter_decay_weight;
    }
}