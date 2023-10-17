use std::sync::Arc;
use nih_plug::editor::Editor;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::views::VStack;
use nih_plug_vizia::widgets::{ParamSlider, ResizeHandle};
use crate::SynthParams;

#[derive(Lens)]
struct Data {
    params: Arc<SynthParams>,
    // peak_meter: Arc<AtomicF32>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (200, 250))
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
                Label::new(cx, "Zwiebers\n brakke synth")
                    .font_family(vec![FamilyOwned::Name(String::from(
                        assets::NOTO_SANS_THIN,
                    ))])
                    .font_size(30.0)
                    .height(Pixels(50.0))
                    .child_top(Stretch(1.0))
                    .child_bottom(Pixels(0.0));

                Label::new(cx, "Gain");
                ParamSlider::new(cx, Data::params, |params| &params.gain);

            }).row_between(Pixels(0.0))
                .child_left(Stretch(1.0))
                .child_right(Stretch(1.0));
        })
}