use nih_plug_vizia::vizia::prelude::*;
use crate::gui::controls::fake_param_button::FakeParamButton;
use crate::gui::controls::wave_controls::WaveControls;
use crate::gui::GuiData;
use crate::params::get_oscillator_array;

pub struct OscillatorControls {}

impl OscillatorControls {
    pub fn new<L>(cx: &mut Context, lens: L) -> Handle<Self>
        where L: 'static + Lens<Target=bool>
    {
        Self {}.build(cx, |cx| {
            ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                VStack::new(cx, |cx| {
                    for i in get_oscillator_array() {
                        let enabled = GuiData::params
                            .map(move |p| p.oscillator_params[i].enabled.value());

                        Binding::new(cx, enabled, move |cx, enabled| {
                            if enabled.get(cx) {
                                WaveControls::new(cx, i).width(Percentage(100.0));
                            }
                        });
                    }

                    Binding::new(cx, lens, |cx, max| {
                        if !max.get(cx) {
                            FakeParamButton::new(
                                cx,
                                |cx| { cx.emit(ControlEvent::AddOscillator) },
                                |cx| { Label::new(cx, "Add") },
                            ).width(Percentage(100.0))
                                .child_space(Stretch(1.0));
                        }
                    });

                }).row_between(Pixels(10.0));
            }).height(Stretch(1.0)).width(Pixels(210.0));
        })
    }
}

impl View for OscillatorControls {}

pub enum ControlEvent {
    AddOscillator,
    RemoveOscillator
}