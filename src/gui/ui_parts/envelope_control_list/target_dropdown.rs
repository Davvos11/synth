use std::sync::Arc;
use nih_plug::params::FloatParam;
use nih_plug::prelude::FloatRange;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::ParamSlider;
use crate::gui::events::ControlEvent;
use crate::params::envelope_target::{get_possible_targets, Target};
use crate::params::SynthParams;

pub struct TargetsList {}

impl View for TargetsList {}

impl TargetsList {
    pub fn new<L>(cx: &mut Context, params: L, envelope_index: usize) -> Handle<Self>
        where L: Lens<Target=Arc<SynthParams>> + Copy,
    {
        Self {}.build(cx, |cx| {
            let data = params.get(cx);
            let length = data.envelope_params[envelope_index].targets
                .lock().expect("Cannot lock envelope targets").targets.len();

            for i in 0..length {
                TargetSelector::new(cx, params, envelope_index, i);
            }
        })
    }
}

pub struct TargetSelector {}

impl View for TargetSelector {
    // fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
    //     event.map(|control_event: &ControlEvent, _meta| match control_event {
    //         SetEnvelopeTarget(_, _, _) => {
    //             dbg!("test");
    //         }
    //         _ => {}
    //     });
    // }
}

impl TargetSelector {
    pub fn new<L>(cx: &mut Context, params: L, envelope_index: usize, target_index: usize) -> Handle<Self>
        where L: Lens<Target=Arc<SynthParams>> + Copy,
    {
        let current_target = params.map(move |p| {
            p.envelope_params[envelope_index].targets
                .lock().expect("Cannot lock envelope targets")
                .targets[target_index]
        });

        Self {}.build(cx, |cx| {
            TargetData::new(current_target.get(cx)).build(cx);

            HStack::new(cx, |cx| {
                Dropdown::new(
                    cx,
                    move |cx| {
                        Label::new(cx, TargetData::target.map(|t| t.to_string()))
                    },
                    move |cx| {
                        for target in get_possible_targets() {
                            Label::new(cx, &target.to_string())
                                .on_press(move |cx| {
                                    cx.emit(ControlEvent::SetEnvelopeTarget(
                                        envelope_index, target_index, target,
                                    ));
                                    cx.emit(PopupEvent::Close);
                                })
                                .width(Stretch(1.0));
                        }
                    },
                ).width(Percentage(50.0));

                Slider::new(cx, TargetData::depth)
                    .on_changing(move |cx, value| {
                        cx.emit(ControlEvent::SetEnvelopeTargetDepth(envelope_index, target_index, value))
                    })
                    .width(Percentage(50.0));
            }).width(Percentage(100.0));
        })
    }
}

#[derive(Lens)]
pub struct TargetData {
    pub target: Target,
    pub depth: f32,
}

impl TargetData {
    pub fn new((target, depth): (Target, f32)) -> Self {
        Self {
            target,
            depth,
        }
    }
}

impl Model for TargetData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|control_event: &ControlEvent, _meta|
            match control_event {
                ControlEvent::SetEnvelopeTarget(_, _, target) => {
                    self.target = *target;
                }
                ControlEvent::SetEnvelopeTargetDepth(_, _, depth) => {
                    self.depth = *depth;
                }
                _ => {}
            }
        );
    }
}