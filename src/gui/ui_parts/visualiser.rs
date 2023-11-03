use std::time::Duration;
use nih_plug::util;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::{ParamSlider, PeakMeter};
use crate::gui::GuiData;
use crate::gui::ui_parts::visualiser::scope::Scope;

mod scope;

pub struct Visualiser {}

impl View for Visualiser {}

impl Visualiser {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            let visual_data_lens = GuiData::visual_data.map(
                |data| data.lock().unwrap().read().clone()
            );


            VStack::new(cx, |cx| {
                VStack::new(cx, |cx| {
                    Label::new(cx, "Volume");
                    ParamSlider::new(cx, GuiData::params, |params| &params.volume);

                    PeakMeter::new(
                        cx,
                        visual_data_lens.clone().map(
                            |d| util::gain_to_db_fast(d.peak_meter)
                        ),
                        Some(Duration::from_millis(600)),
                    );
                })
                    .row_between(Pixels(0.0))
                    .child_left(Stretch(1.0))
                    .child_right(Stretch(1.0));

                Scope::new(cx, visual_data_lens.clone())
                    .height(Pixels(200.0));

                VStack::new(cx, |_cx| {

                });
            })
                .row_between(Pixels(0.0));
        })
    }
}