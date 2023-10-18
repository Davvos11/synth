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
            Data::peak_meter
                .map(|peak_meter| util::gain_to_db(peak_meter.load(Ordering::Relaxed))),
            Some(Duration::from_millis(600)),
        );
    }).row_between(Stretch(2.0))
}