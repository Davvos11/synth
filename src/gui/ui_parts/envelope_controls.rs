use nih_plug_vizia::vizia::context::Context;
use crate::gui::GuiData;
use nih_plug_vizia::vizia::prelude::*;
use crate::gui::components::knob::ParamKnob;
use crate::gui::ui_parts::envelope_controls::envelope_graph::Graph;

mod envelope_graph;

pub struct EnvelopeControls {}

impl View for EnvelopeControls {}

impl EnvelopeControls {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            VStack::new(cx, |cx| {
                Graph::new(cx, GuiData::params)
                    .width(Percentage(100.0))
                    .height(Pixels(80.0));

                HStack::new(cx, |cx| {
                    ParamKnob::new(cx, GuiData::params, |params| &params.attack,
                                   false, Some("Attack"), false);

                    ParamKnob::new(cx, GuiData::params, |params| &params.decay,
                                   false, Some("Decay"), false);

                    ParamKnob::new(cx, GuiData::params, |params| &params.sustain,
                                   false, Some("Sustain"), false);

                    ParamKnob::new(cx, GuiData::params, |params| &params.release,
                                   false, Some("Release"), false);
                })
                    .bottom(Pixels(5.0))
                    .top(Pixels(5.0));
            }).row_between(Pixels(5.0))
                .child_left(Stretch(1.0))
                .child_right(Stretch(1.0))
                .height(Pixels(0.0))
                .border_color(Color::black())
                .border_width(Pixels(1.0));
        })
    }
}
