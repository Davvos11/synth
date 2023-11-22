use nih_plug_vizia::vizia::prelude::*;
use crate::gui::components::fake_param_button::FakeParamButton;
use crate::gui::events::ControlEvent;
use crate::gui::GuiData;
use crate::gui::ui_parts::envelope_control_list::envelope_controls::EnvelopeControls;
use crate::utils::get_envelope_array;

mod envelope_controls;
mod envelope_graph;
mod target_dropdown;

pub struct EnvelopeControlList {}

impl EnvelopeControlList {
    pub fn new<L>(cx: &mut Context, max_envelopes: L) -> Handle<Self>
        where L: 'static + Lens<Target=bool>
    {
        Self {}.build(cx, |cx| {
            ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                VStack::new(cx, |cx| {
                    for i in get_envelope_array() {
                        let enabled = GuiData::params
                            .map(move |p| p.envelope_params[i].enabled.value());

                        Binding::new(cx, enabled, move |cx, enabled| {
                            if enabled.get(cx) {
                                EnvelopeControls::new(cx, i).width(Percentage(100.0));
                            }
                        });
                    }

                    Binding::new(cx, max_envelopes, |cx, max| {
                        if !max.get(cx) {
                            FakeParamButton::new(
                                cx,
                                |cx| { cx.emit(ControlEvent::AddEnvelope) },
                                |cx| { Label::new(cx, "Add") },
                            ).width(Percentage(100.0))
                                .child_space(Stretch(1.0));
                        }
                    });

                }).row_between(Pixels(10.0)).width(Percentage(90.0));
            }).height(Stretch(1.0)).width(Pixels(220.0));
        })
    }
}

impl View for EnvelopeControlList {

}