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
        canvas.stroke_path(&mut outline, &vg::Paint::color(Color::black().into()));

        // Draw the waveform
        let mut samples = self.visual_data.get(cx).samples;

        let mut wave = vg::Path::new();
        let baseline = bounds.y + bounds.h / 2.0;
        wave.move_to(bounds.x, baseline);

        // Calculate chunk size
        let chunk_size = (samples.len() as f32 / bounds.w) as usize;
        // Ignore first samples so they fit in n / chunk_size chunks
        let overflow = (samples.len() as f32 % bounds.w) as usize;
        samples.drain(0..overflow);

        for (x, chunk) in samples.chunks(chunk_size).enumerate() {
            let chunk_average = chunk.iter().sum::<f32>() / (chunk.len() as f32);

            let x = bounds.x + x as f32;
            let y = baseline + chunk_average * (bounds.h / 2.0);

            wave.line_to(x, y);
        }

        let mut paint = Paint::color(Color::black().into());
        paint.set_line_width(1.0);
        canvas.stroke_path(&mut wave, &paint);
    }
}
