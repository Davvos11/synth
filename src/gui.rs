use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use nih_plug::editor::Editor;
use nih_plug::prelude::{GuiContext, ParamSetter};
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::ResizeHandle;
use crate::gui::events::{add_item, ControlEvent};
use crate::gui::ui_parts::envelope_control_list::EnvelopeControlList;
use crate::gui::ui_parts::oscillator_control_list::OscillatorControlList;
use crate::gui::ui_parts::visualiser::Visualiser;
use crate::SynthParams;
use crate::process::visual_data::VisualData;

mod components;
mod ui_parts;
mod events;

#[derive(Lens)]
pub struct GuiData {
    params: Arc<SynthParams>,
    visual_data: Arc<Mutex<triple_buffer::Output<VisualData>>>,
    gui_context: Arc<dyn GuiContext>,
    // TODO data structure to generalise this?
    max_oscillators: Arc<AtomicBool>,
    max_envelopes: Arc<AtomicBool>,
}

impl Model for GuiData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        let setter = ParamSetter::new(self.gui_context.as_ref());

        event.map(|control_event: &ControlEvent, _meta| match control_event {
            ControlEvent::AddOscillator => {
                add_item(&self.params.oscillator_params, setter, self.max_oscillators.clone());
            }
            ControlEvent::RemoveOscillator => {
                self.max_oscillators.store(false, Ordering::Relaxed);
            }

            ControlEvent::AddEnvelope => {
                add_item(&self.params.envelope_params, setter, self.max_envelopes.clone());
            }
            ControlEvent::RemoveEnvelope => {
                self.max_envelopes.store(false, Ordering::Relaxed);
            }
        });

    }
}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (700, 550))
}

pub(crate) fn create(
    params: Arc<SynthParams>,
    editor_state: Arc<ViziaState>,
    visual_data: Arc<Mutex<triple_buffer::Output<VisualData>>>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(
        editor_state,
        ViziaTheming::Custom,
        move |cx, gui_cx| {
            assets::register_noto_sans_light(cx);
            assets::register_noto_sans_thin(cx);

            cx.add_stylesheet("assets/style.css").expect("Failed to load stylesheet");

            GuiData {
                params: params.clone(),
                visual_data: visual_data.clone(),
                gui_context: gui_cx,
                max_oscillators: Arc::new(AtomicBool::new(false)),
                max_envelopes: Arc::new(AtomicBool::new(false)),
            }.build(cx);

            let max_oscillators = GuiData::max_oscillators.map(|m|{
                m.load(Ordering::Relaxed)
            });
            let max_envelopes = GuiData::max_envelopes.map(|m|{
                m.load(Ordering::Relaxed)
            });

            ResizeHandle::new(cx);

            VStack::new(cx, |cx| {
                Label::new(cx, "ZwieberSynth")
                    .font_family(vec![FamilyOwned::Name(String::from(
                        assets::NOTO_SANS_THIN,
                    ))])
                    .font_size(30.0)
                    .height(Pixels(25.0))
                    .bottom(Pixels(20.0));

                HStack::new(cx, |cx| {
                    OscillatorControlList::new(cx, max_oscillators);

                    EnvelopeControlList::new(cx, max_envelopes);

                    Visualiser::new(cx);
                }).col_between(Pixels(20.0));
            }).child_space(Stretch(1.0))
                .top(Pixels(10.0))
                .bottom(Pixels(10.0))
                .child_bottom(Pixels(0.0))
                .child_top(Pixels(0.0));
        })
}

