use std::sync::Arc;
use nih_plug_vizia::assets;
use nih_plug_vizia::vizia::prelude::*;
use crate::gui::components::fake_param_button::FakeParamButton;
use crate::gui::components::fake_param_slider::{FakeParamSlider, SliderHandle};
use crate::gui::events::ControlEvent;
use crate::params::envelope_target::{EnvelopeTargets, get_possible_targets, Target};
use crate::params::SynthParams;

pub struct TargetsList<L>
    where L: Lens<Target=Arc<SynthParams>> + Copy,
{
    params: L,
    envelope_index: usize,
}

impl<L> TargetsList<L>
    where L: Lens<Target=Arc<SynthParams>> + Copy,
{
    pub fn new(cx: &mut Context, params: L, envelope_index: usize) -> Handle<Self>
    {
        Self {
            params,
            envelope_index,
        }.build(cx, |cx| {
            let data = params.get(cx);
            EnvelopeTargets::from(
                data.envelope_params[envelope_index].targets
                    .lock().expect("Cannot lock envelope targets")
            ).build(cx);

            HStack::new(cx, |cx| {
                Label::new(cx, "Targets:");
                FakeParamButton::new(cx,
                                     move |cx| {
                                         cx.emit(ControlEvent::AddEnvelopeTarget);
                                     },
                                     |cx| Label::new(cx, "+"))
                    .left(Stretch(1.0))
                    .right(Pixels(0.0));
            })
                .child_space(Stretch(1.0))
                .child_left(Pixels(0.0))
                .col_between(Pixels(5.0));

            List::new(cx, EnvelopeTargets::targets, move |cx, i, _| {
                // TODO use lens instead of index
                TargetSelector::new(cx, params, envelope_index, i);
            })
                .width(Stretch(1.0))
                .row_between(Pixels(1.0));
        })
    }
}

impl<L> View for TargetsList<L>
    where L: Lens<Target=Arc<SynthParams>> + Copy,
{
    #[allow(clippy::single_match)]
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|control_event: &ControlEvent, _meta|
            match control_event {
                ControlEvent::AddEnvelopeTarget | ControlEvent::RemoveEnvelopeTarget(_) => {
                    let data = self.params.get(cx);
                    let mut targets = data.envelope_params[self.envelope_index].targets
                        .lock().expect("Cannot lock envelope targets");
                    targets.targets = EnvelopeTargets::targets.get(cx);
                }
                _ => {}
            }
        );
    }
}

pub struct TargetSelector {}

impl View for TargetSelector {
    fn element(&self) -> Option<&'static str> {
        Some("target-selector")
    }
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
                ZStack::new(cx, |cx| {
                    Dropdown::new(
                        cx,
                        move |cx| {
                            Label::new(cx, TargetData::target.map(|t| t.to_string()))
                        },
                        move |cx| {
                            let max_index = TargetData::possible_targets.get(cx).len() - 1;
                            let active_target = TargetData::target.get(cx);
                            List::new(cx, TargetData::possible_targets, move |cx, i, get_target| {
                                let target = get_target.get(cx);
                                Label::new(cx, &target.to_string())
                                    .font_family(
                                        if target == active_target { vec![FamilyOwned::Name(String::from(assets::NOTO_SANS_BOLD))] } else { vec![FamilyOwned::Name(String::from(assets::NOTO_SANS_LIGHT))] })
                                    .on_press(move |cx| {
                                        cx.emit(ControlEvent::SetEnvelopeTarget(
                                            envelope_index, target_index, target,
                                        ));
                                        cx.emit(PopupEvent::Close);
                                    })
                                    .width(Stretch(1.0));
                                if i != max_index {
                                    Element::new(cx).class("separator");
                                }
                            });
                        },
                    ).width(Percentage(100.0));

                    Label::new(cx, "v")
                        .class("dropdown-icon");
                }).width(Percentage(60.0));


                // TODO get default value from somewhere?
                FakeParamSlider::new(cx, TargetData::depth, 1.0)
                    .on_changing(move |cx, value| {
                        cx.emit(ControlEvent::SetEnvelopeTargetDepth(envelope_index, target_index, value))
                    })
                    .width(Stretch(1.0));

                FakeParamButton::new(cx,
                                     move |cx| {
                                         cx.emit(ControlEvent::RemoveEnvelopeTarget(target_index))
                                     },
                                     |cx| {
                                         Label::new(cx, "-")
                                     },
                );
            })
                .width(Percentage(100.0))
                .col_between(Pixels(2.0));
        })
    }
}

#[derive(Lens)]
pub struct TargetData {
    pub target: Target,
    pub depth: f32,
    pub possible_targets: Vec<Target>,
}

impl TargetData {
    pub fn new((target, depth): (Target, f32)) -> Self {
        Self {
            target,
            depth,
            possible_targets: get_possible_targets(),
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