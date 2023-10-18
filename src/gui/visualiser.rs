use std::time::Duration;
use nih_plug::util;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::PeakMeter;
use crate::gui::GuiData;


pub fn visualiser(cx: &mut Context) -> Handle<VStack> {
    // TODO can this be shorter / clearer / less complicated?
    let gain = GuiData::visual_data.map(
        |data| {
            let mut visual_data = data.lock().unwrap();
            let visual_data = visual_data.read();
            util::gain_to_db_fast(visual_data.peak_meter)
        });

    VStack::new(cx, |cx| {
        PeakMeter::new(
            cx,
            gain,
            Some(Duration::from_millis(600)),
        );
    }).row_between(Stretch(2.0))
}