use std::cmp::min;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;
use nih_plug_vizia::vizia::vg::Paint;
use crate::process::visual_data::VisualData;

pub struct Scope<L>
    where L: Lens<Target=VisualData>
{
    visual_data: L,
}

impl<L> Scope<L>
    where L: Lens<Target=VisualData>
{
    pub fn new(cx: &mut Context, visual_data: L) -> Handle<Self>
    {
        Self {
            visual_data
        }.build(cx, |_| {})
    }
}

impl<L> View for Scope<L>
    where L: Lens<Target=VisualData>
{
    fn element(&self) -> Option<&'static str> {
        Some("Scope")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {

        // Get the bounding box of the current view.
        let bounds = cx.bounds();

        // Create a box
        let mut outline = vg::Path::new();
        outline.rect(bounds.x, bounds.y, bounds.w, bounds.h);
        // TODO get color from context instead of putting black
        let mut paint = Paint::color(Color::black().into());
        paint.set_line_width(1.0);
        paint.set_anti_alias(false);
        canvas.stroke_path(&mut outline, &paint);

        // Draw the waveform
        let visual_data = self.visual_data.get(cx);
        let sample_data = visual_data.samples;
        let mut samples = &sample_data[..];

        let mut wave = vg::Path::new();
        let baseline = bounds.y + bounds.h / 2.0;
        wave.move_to(bounds.x, baseline);

        let start_points = find_wave_starts(samples);
        // Crop to screen
        // TODO cache crop size or base on note that is playing
        samples = crop_to_screen(samples, start_points, bounds.w, 2);

        // Zoom out to screen
        let chunk_size = if samples.len() as f32 > bounds.w {
            let chunk_size = (samples.len() as f32 / bounds.w) as usize;
            // Ignore first samples so they fit in n / chunk_size chunks
            let overflow = (samples.len() as f32 % bounds.w) as usize;
            samples = &samples[overflow..];
            chunk_size
        } else {
            1
        };

        // Draw to screen (use steps of `chunk_size`) if there are more datapoints than pixels
        for (x, i) in (0..samples.len()).step_by(chunk_size).enumerate() {
            let x = bounds.x + x as f32;
            let y = baseline + samples[i] * (bounds.h / 2.0);

            wave.line_to(x, y);
        }

        let mut paint = Paint::color(Color::black().into());
        paint.set_line_width(1.0);
        canvas.stroke_path(&mut wave, &paint);
    }
}

fn find_wave_starts(samples: &[f32]) -> Vec<usize> {
    let mut zero_crossings = Vec::new();
    for i in 1..samples.len() {
        if samples[i - 1] * samples[i] < 0.0 && samples[i - 1] < 0.0 {
            zero_crossings.push(i);
        }
    }

    zero_crossings
}

fn crop_to_screen(samples: &[f32], mut start_points: Vec<usize>, width: f32, min_waves: usize) -> &[f32] {
    start_points.reverse();

    if let Some(end_index) = start_points.first() {
        if let Some(mut start_index) = start_points.get(min_waves) {

            // Calculate the smallest amount of waves needed to fill the screen
            let mut waves = min_waves;
            let width = width as usize;
            while (end_index - start_index) <= width {
                if let Some(index) = start_points.get(waves) {
                    start_index = index;
                    waves += 2;
                } else {
                    break;
                }
            }

            let result = &samples[*start_index..=*end_index];
            return if waves > min_waves {
                let start = result.len() - min(result.len(), width);
                &result[start..]
            } else {
                result
            }
        }
    }

    samples
}