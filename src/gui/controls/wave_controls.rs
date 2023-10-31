use nih_plug_vizia::vizia::prelude::*;
use crate::gui::controls::selector::{ButtonLabel, get_enum_name, Selector};
use crate::gui::GuiData;
use crate::gui::knob::ParamKnob;
use crate::note::WaveKind;

pub struct WaveControls {}

impl View for WaveControls {}

impl WaveControls {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self{}.build(cx, |cx| {
            let display_pwm = GuiData::params
                .map(|p| p.wave_kind.value() == WaveKind::Square);

            VStack::new(cx, |cx| {
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
                    }).child_space(Pixels(1.0))
                        .col_between(Pixels(5.0));
                })
                    .row_between(Pixels(0.0))
                    .child_left(Stretch(1.0))
                    .child_right(Stretch(1.0))
                    .space(Pixels(5.0))
                    .bottom(Pixels(10.0));

            })
                .border_color(Color::black())
                .border_width(Pixels(1.0))
                .width(Percentage(100.0))
                .height(Pixels(10.0)); // I hate that this is the way to fix this
        })
    }
}