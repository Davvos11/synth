use std::time::Duration;
use nih_plug::util;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::PeakMeter;
use crate::gui::GuiData;
use crate::gui::visualiser::scope::Scope;

mod scope;

pub fn visualiser(cx: &mut Context)
    -> Handle<VStack> {
    // TODO can this be shorter / clearer / less complicated?
    let gain = GuiData::visual_data.map(
        |data| {
            let mut visual_data = data.lock().unwrap();
            let visual_data = visual_data.read();
            util::gain_to_db_fast(visual_data.peak_meter)
        });

    let samples = GuiData::visual_data.map(
        |data| {
            let mut visual_data = data.lock().unwrap();
            let visual_data = visual_data.read();
            visual_data.samples.clone()
        });


    VStack::new(cx, |cx| {
        PeakMeter::new(
            cx,
            gain,
            Some(Duration::from_millis(600)),
        );

        Scope::new(cx, samples);
    }).row_between(Pixels(15.0))
}