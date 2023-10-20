use nih_plug_vizia::vizia::context::Context;
use nih_plug_vizia::widgets::ParamSlider;
use crate::gui::GuiData;
use nih_plug_vizia::vizia::prelude::*;
use crate::gui::controls::selector::Selector;

mod selector;

pub fn controls(cx: &mut Context) -> Handle<VStack> {
    VStack::new(cx, |cx| {
        Label::new(cx, "Wave");
        Selector::new(cx, GuiData::params, |p|&p.wave_kind);

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
        .child_right(Stretch(1.0))
}