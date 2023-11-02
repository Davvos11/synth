use nih_plug_vizia::vizia::context::Context;
use nih_plug_vizia::widgets::ParamSlider;
use crate::gui::GuiData;
use nih_plug_vizia::vizia::prelude::*;

pub mod selector;
pub mod fake_param_button;
pub mod oscillator_controls;

pub struct Controls {}

impl View for Controls {}

impl Controls {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Attack");
                ParamSlider::new(cx, GuiData::params, |params| &params.attack);

                Label::new(cx, "Decay");
                ParamSlider::new(cx, GuiData::params, |params| &params.decay);

                Label::new(cx, "Sustain");
                ParamSlider::new(cx, GuiData::params, |params| &params.sustain);

                Label::new(cx, "Release");
                ParamSlider::new(cx, GuiData::params, |params| &params.release);
            }).row_between(Pixels(0.0))
                .child_left(Stretch(1.0))
                .child_right(Stretch(1.0));
        })
    }
}
