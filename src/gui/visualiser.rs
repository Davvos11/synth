use std::time::Duration;
use nih_plug::util;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::PeakMeter;
use crate::gui::GuiData;
use crate::gui::visualiser::scope::Scope;

mod scope;

pub fn visualiser(cx: &mut Context)
                  -> Handle<VStack> {
    let visual_data_lens = GuiData::visual_data.map(
        |data| data.lock().unwrap().read().clone()
    );


    VStack::new(cx, |cx| {
        PeakMeter::new(
            cx,
            visual_data_lens.clone().map(
                |d| util::gain_to_db_fast(d.peak_meter)
            ),
            Some(Duration::from_millis(600)),
        );

        Scope::new(cx, visual_data_lens);
    }).row_between(Pixels(15.0))
}