use std::sync::Arc;
use nih_plug::params::Param;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;
use nih_plug_vizia::vizia::vg::Paint;
use crate::params::SynthParams;
use crate::process::envelope::Adsr;

pub struct Graph<L>
    where L: Lens<Target=Arc<SynthParams>>
{
    params: L,
}

impl<L> Graph<L>
    where L: Lens<Target=Arc<SynthParams>>
{
    pub fn new(cx: &mut Context, params: L) -> Handle<Self> {
        Self {
            params,
        }.build(cx, |_| {})
    }
}

impl<L> View for Graph<L>
    where L: Lens<Target=Arc<SynthParams>>
{
    fn element(&self) -> Option<&'static str> {
        Some("envelope-graph")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        // Get the bounding box of the current view.
        let bounds = cx.bounds();
        let baseline = bounds.y + bounds.h;

        // Create a box
        let mut bottom_line = vg::Path::new();
        bottom_line.move_to(bounds.x, baseline);
        bottom_line.line_to(bounds.x + bounds.w, baseline);
        // TODO get color from context instead of putting black
        let mut paint = Paint::color(Color::black().into());
        paint.set_line_width(1.0);
        paint.set_anti_alias(false);
        canvas.stroke_path(&mut bottom_line, &paint);

        // Get ADSR
        let params = self.params.clone().map(|p| {
            Adsr::new(
                p.attack.modulated_normalized_value(),
                p.decay.modulated_normalized_value(),
                p.sustain.modulated_normalized_value(),
                p.release.modulated_normalized_value(),
            )
        }).get(cx);

        // Draw envelope curve
        let mut wave = vg::Path::new();
        let mut x = bounds.x;
        wave.move_to(x, baseline);
        // Attack
        x += params.attack() * bounds.w / 3.0;
        wave.line_to(x, bounds.y);
        // Extra pixel to remove spike at the top of graph
        x += 1.0;
        wave.line_to(x, bounds.y);
        // Decay & sustain
        x += params.decay() * bounds.w / 3.0;
        wave.line_to(x, baseline - params.sustain() * bounds.h);
        // Release
        x += params.release() * bounds.w / 3.0;
        wave.line_to(x, baseline);

        // Draw
        let mut paint = Paint::color(Color::black().into());
        paint.set_line_width(1.0);
        canvas.stroke_path(&mut wave, &paint);

    }
}