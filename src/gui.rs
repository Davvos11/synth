use std::sync::Arc;
use nih_plug::editor::Editor;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::{ResizeHandle};
use crate::gui::controls::controls;
use crate::SynthParams;

mod controls;

#[derive(Lens)]
struct Data {
    params: Arc<SynthParams>,
    // peak_meter: Arc<AtomicF32>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (500, 350))
}

pub(crate) fn create(
    params: Arc<SynthParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(
        editor_state,
        ViziaTheming::Custom,
        move |cx, _| {
            assets::register_noto_sans_light(cx);
            assets::register_noto_sans_thin(cx);

            Data {
                params: params.clone()
            }.build(cx);

            ResizeHandle::new(cx);

            VStack::new(cx, |cx| {
                Label::new(cx, "ZwieberSynth")
                    .font_family(vec![FamilyOwned::Name(String::from(
                        assets::NOTO_SANS_THIN,
                    ))])
                    .font_size(30.0)
                    .height(Pixels(25.0));

                HStack::new(cx, |cx| {
                    controls(cx);

                }).col_between(Pixels(20.0));
            }).child_space(Stretch(1.0));

        })
}