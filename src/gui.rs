use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use nih_plug::editor::Editor;
use nih_plug::prelude::{GuiContext, ParamSetter};
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::ResizeHandle;
use crate::gui::controls::Controls;
use crate::gui::visualiser::Visualiser;
use crate::SynthParams;
use crate::process::visual_data::VisualData;
use crate::gui::oscillator_control_list::{ControlEvent, OscillatorControlList};

mod controls;
mod visualiser;
mod grid;
mod knob;
mod oscillator_control_list;

#[derive(Lens)]
pub struct GuiData {
    params: Arc<SynthParams>,
    visual_data: Arc<Mutex<triple_buffer::Output<VisualData>>>,
    gui_context: Arc<dyn GuiContext>,
    max_oscillators: Arc<AtomicBool>,
}

impl Model for GuiData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        let setter = ParamSetter::new(self.gui_context.as_ref());

        event.map(|control_event: &ControlEvent, _meta| match control_event {
            ControlEvent::AddOscillator => {
                let mut all_enabled = true;
                let mut set = false;
                // Loop over oscillators, setting the first that is disabled
                // and checking if all have been enabled or not
                for param in &self.params.oscillator_params {
                    // Find first oscillator that is disabled
                    if !param.enabled.value() {
                        if set {
                            all_enabled = false;
                            break;
                        }
                        // Enable
                        setter.begin_set_parameter(&param.enabled);
                        setter.set_parameter(&param.enabled, true);
                        setter.end_set_parameter(&param.enabled);
                        // Go to next loop
                        set = true;
                        continue
                    }
                }

                if all_enabled {
                    self.max_oscillators.store(true, Ordering::Relaxed);
                }
            }
            ControlEvent::RemoveOscillator => {
                self.max_oscillators.store(false, Ordering::Relaxed);
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
            }.build(cx);

            let l = GuiData::max_oscillators.map(|m|{
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
                    OscillatorControlList::new(cx, l);

                    Controls::new(cx);

                    Visualiser::new(cx);
                }).col_between(Pixels(20.0));
            }).child_space(Stretch(1.0))
                .top(Pixels(10.0))
                .bottom(Pixels(10.0))
                .child_bottom(Pixels(0.0))
                .child_top(Pixels(0.0));
        })
}

