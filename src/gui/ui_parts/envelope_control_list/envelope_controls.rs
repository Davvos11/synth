use nih_plug_vizia::vizia::context::Context;
use crate::gui::GuiData;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::{ParamButton, ParamButtonExt};
use crate::gui::components::knob::ParamKnob;
use crate::gui::components::param_button_wrapper::ParamButtonWrapper;
use crate::gui::events::ControlEvent;
use crate::gui::ui_parts::envelope_control_list::envelope_graph::Graph;
use crate::gui::ui_parts::envelope_control_list::target_dropdown::TargetsList;

pub struct EnvelopeControls {}

impl View for EnvelopeControls {}

impl EnvelopeControls {
    pub fn new(cx: &mut Context, index: usize) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            VStack::new(cx, move |cx| {
                ZStack::new(cx, |cx| {
                    Graph::new(cx, GuiData::params, index)
                        .width(Percentage(100.0))
                        .height(Percentage(100.0));

                    ParamButtonWrapper::new(
                        cx,
                        |cx| {
                            ParamButton::new(cx, GuiData::params, move |p| &p.envelope_params[index].enabled)
                                .with_label("X");
                        },
                        |cx| cx.emit(ControlEvent::RemoveEnvelope),
                    ).width(Pixels(0.0))
                        .left(Stretch(1.0))
                        .right(Pixels(5.0))
                        .top(Pixels(5.0));
                })
                    .width(Percentage(100.0))
                    .height(Pixels(80.0));

                HStack::new(cx, move |cx| {
                    ParamKnob::new(cx, GuiData::params, move |p| &p.envelope_params[index].attack,
                                   false, Some("Attack"), false);

                    ParamKnob::new(cx, GuiData::params, move |p| &p.envelope_params[index].decay,
                                   false, Some("Decay"), false);

                    ParamKnob::new(cx, GuiData::params, move |p| &p.envelope_params[index].sustain,
                                   false, Some("Sustain"), false);

                    ParamKnob::new(cx, GuiData::params, move |p| &p.envelope_params[index].release,
                                   false, Some("Release"), false);
                })
                    .bottom(Pixels(5.0))
                    .top(Pixels(5.0));

                TargetsList::new(cx, GuiData::params, index)
                    .width(Percentage(100.0))
                    .child_left(Pixels(5.0))
                    .child_right(Pixels(5.0))
                    .row_between(Pixels(5.0))
                    .bottom(Pixels(10.0));
            }).row_between(Pixels(5.0))
                .child_left(Stretch(1.0))
                .child_right(Stretch(1.0))
                .height(Pixels(0.0))
                .border_color(Color::black())
                .border_width(Pixels(1.0));
        })
    }
}
