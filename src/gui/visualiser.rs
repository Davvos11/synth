use std::sync::atomic::Ordering;
use std::time::Duration;
use nih_plug::util;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::PeakMeter;
use crate::gui::Data;

pub fn visualiser(cx: &mut Context) -> Handle<VStack> {
    VStack::new(cx, |cx| {
        PeakMeter::new(
            cx,
            Data::data
                .map(|data| util::gain_to_db(data.peak_meter.load(Ordering::Relaxed))),
            Some(Duration::from_millis(600)),
        );
    }).row_between(Stretch(2.0))
}