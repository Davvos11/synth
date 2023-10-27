use nih_plug_vizia::vizia::context::Context;
use nih_plug_vizia::widgets::ParamSlider;
use crate::gui::GuiData;
use nih_plug_vizia::vizia::prelude::*;
use crate::gui::controls::selector::{ButtonLabel, get_enum_name, Selector};
use crate::gui::knob::ParamKnob;
use crate::note::WaveKind;

mod selector;

pub struct Controls {}

impl View for Controls {}

impl Controls {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            let display_pwm = GuiData::params
                .map(|p| p.wave_kind.value() == WaveKind::Square);

            VStack::new(cx, |cx| {
                Label::new(cx, "Wave").bottom(Pixels(5.0));

                HStack::new(cx, |cx| {
                    Selector::new(cx, GuiData::params, |p| &p.wave_kind,
                                  |v| ButtonLabel::Text(get_enum_name(v)),
                    );
                    Binding::new(cx, display_pwm, |cx, display| {
                        if display.get(cx) {
                            ParamKnob::new(cx, GuiData::params, |p| &p.pulse_width, true);
                        }
                    });
                }).child_space(Stretch(1.0))
                    .col_between(Pixels(5.0));

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
