use std::sync::{Arc, Mutex};
use nih_plug::editor::Editor;
use nih_plug_vizia::{assets, create_vizia_editor, ViziaState, ViziaTheming};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::{ParamSlider, ResizeHandle};
use crate::gui::controls::Controls;
use crate::gui::visualiser::Visualiser;
use crate::SynthParams;
use crate::process::visual_data::VisualData;
use crate::gui::controls::wave_controls::WaveControls;

mod controls;
mod visualiser;
mod grid;
mod knob;

#[derive(Lens)]
pub struct GuiData {
    params: Arc<SynthParams>,
    visual_data: Arc<Mutex<triple_buffer::Output<VisualData>>>,
}

impl Model for GuiData {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (700, 350))
}

pub(crate) fn create(
    params: Arc<SynthParams>,
    editor_state: Arc<ViziaState>,
    visual_data: Arc<Mutex<triple_buffer::Output<VisualData>>>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(
        editor_state,
        ViziaTheming::Custom,
        move |cx, _| {
            assets::register_noto_sans_light(cx);
            assets::register_noto_sans_thin(cx);

            cx.add_stylesheet("assets/style.css").expect("Failed to load stylesheet");

            GuiData {
                params: params.clone(),
                visual_data: visual_data.clone(),
            }.build(cx);

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
                    WaveControls::new(cx).width(Pixels(180.0));

                    Controls::new(cx);

                    VStack::new(cx, |cx| {
                        Label::new(cx, "Volume");
                        ParamSlider::new(cx, GuiData::params, |params| &params.volume);

                        Visualiser::new(cx);
                    }).row_between(Pixels(0.0))
                        .child_left(Stretch(1.0))
                        .child_right(Stretch(1.0));
                }).col_between(Pixels(20.0));
            }).child_space(Stretch(1.0))
                .top(Pixels(10.0))
                .bottom(Pixels(10.0))
                .child_bottom(Pixels(0.0))
                .child_top(Pixels(0.0));
        })
}