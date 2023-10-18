use std::sync::Arc;
use nih_plug::prelude::AtomicF32;
use nih_plug::util;

pub struct SynthData {
    data: Arc<VisualData>,
    // Depends on sample rate:
    peak_meter_decay_weight: f32,
}

impl Default for SynthData {
    fn default() -> Self {
        Self {
            data: Arc::new(VisualData::default()),
            peak_meter_decay_weight: 1.0,
        }
    }
}

pub struct VisualData {
    pub peak_meter: AtomicF32,
    pub samples: Vec<AtomicF32>,
}

impl Default for VisualData {
    fn default() -> Self {
        Self {
            peak_meter: AtomicF32::from(util::MINUS_INFINITY_DB),
            samples: Vec::with_capacity(64),
        }
    }
}

impl SynthData {
    pub fn set_visual_data(&mut self, new_sample: f32) {
        let amplitude = new_sample.abs();
        let current_peak_meter = self.data.peak_meter.load(std::sync::atomic::Ordering::Relaxed);
        let new_peak_meter = if amplitude > current_peak_meter {
            amplitude
        } else {
            current_peak_meter * self.peak_meter_decay_weight
                + amplitude * (1.0 - self.peak_meter_decay_weight)
        };

        self.data.peak_meter
            .store(new_peak_meter, std::sync::atomic::Ordering::Relaxed)
    }

    pub fn set_peak_meter_decay_weight(&mut self, peak_meter_decay_weight: f32) {
        self.peak_meter_decay_weight = peak_meter_decay_weight;
    }

    pub fn get_arc_clone(&self) -> Arc<VisualData> {
        self.data.clone()
    }
}