use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::{ParamButton, ParamButtonExt};
use crate::gui::components::knob::ParamKnob;
use crate::gui::components::param_button_wrapper::ParamButtonWrapper;
use crate::gui::components::selector::{ButtonLabel, get_enum_name, Selector};
use crate::gui::GuiData;
use crate::gui::ui_parts::oscillator_control_list::ControlEvent;
use crate::process::note::WaveKind;

pub struct OscillatorControls {}

impl View for OscillatorControls {}

impl OscillatorControls {
    pub fn new(cx: &mut Context, i: usize) -> Handle<Self> {
        Self {}.build(cx, move |cx| {
            let display_pwm = GuiData::params
                .map(move |p| p.oscillator_params[i].wave_kind.value() == WaveKind::Square);

            VStack::new(cx, move |cx| {
                VStack::new(cx, move |cx| {
                    HStack::new(cx, |cx| {
                        // Label::new(cx, &format!("Oscillator {i}"));
                        Selector::new(cx, GuiData::params, move |p| &p.oscillator_params[i].wave_kind,
                                      |v| ButtonLabel::Text(get_enum_name(v)),
                        );

                        ParamButtonWrapper::new(
                            cx,
                            |cx| {
                                ParamButton::new(cx, GuiData::params, move |p| &p.oscillator_params[i].enabled)
                                    .with_label("X");
                            },
                            |cx| cx.emit(ControlEvent::RemoveOscillator),
                        ).width(Pixels(0.0))
                            .top(Pixels(0.0));
                    }).child_top(Stretch(1.0))
                        .child_bottom(Stretch(1.0))
                        .col_between(Stretch(1.0))
                        .width(Percentage(100.0))
                        .bottom(Pixels(5.0));

                    HStack::new(cx, move |cx| {
                        ParamKnob::new(cx, GuiData::params, move |p| &p.oscillator_params[i].volume,
                                       false, Some("Volume"), false);

                        ParamKnob::new(cx, GuiData::params, move |p| &p.oscillator_params[i].transpose,
                                       true, Some("Transpose"), false);

                        ParamKnob::new(cx, GuiData::params, move |p| &p.oscillator_params[i].detune,
                                       true, Some("Detune"), false);
                    })
                        .class("osc-buttons")
                        .child_space(Pixels(1.0))
                        .col_between(Pixels(5.0));

                    HStack::new(cx, move |cx| {
                        Binding::new(cx, display_pwm, move |cx, display| {
                            if display.get(cx) {
                                ParamKnob::new(cx, GuiData::params, move |p| &p.oscillator_params[i].pulse_width,
                                               true, Some("PW"), false);
                            }
                        });
                    })
                        .class("osc-buttons")
                        .child_space(Pixels(1.0))
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

