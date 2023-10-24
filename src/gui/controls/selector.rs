use std::rc::Rc;
use enum_iterator::{all, All, Sequence};
use nih_plug::prelude::{Enum, EnumParam, Param};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::param_base::ParamWidgetBase;
use crate::gui::grid::{Grid, GridVerticalModifiers};

enum SelectorEvent<T>
    where T: PartialEq + Enum + 'static + Sequence + Send + Copy
{
    Select(T)
}

pub struct Selector<T>
    where T: PartialEq + Enum + 'static + Sequence + Send + Copy
{
    pub param_base: ParamWidgetBase,
    // TODO this is nasty, but otherwise the crab gets mad at me and I am too stupid to fix it properly
    pub data: Option<T>,
}

impl<T> Selector<T>
    where T: PartialEq + Enum + 'static + Sequence + Send + Copy + Clone
{
    pub fn new<L, Params, FMap>(cx: &mut Context,
                                params: L,
                                params_to_param: FMap,
                                param_to_label: fn(T) -> ButtonLabel,
    ) -> Handle<Self>
        where L: Lens<Target=Params> + Clone,
              Params: 'static,
              FMap: Fn(&Params) -> &EnumParam<T> + Copy + 'static,
    {
        Self {
            param_base: ParamWidgetBase::new(cx, params.clone(), params_to_param),
            data: None,
        }.build(cx, move |cx: &mut Context| {
            let mut buttons = Vec::new();
            let options: All<T> = all::<T>();

            for option in options {
                let option_rc0 = Rc::new(option);
                let option_rc1 = Rc::clone(&option_rc0);
                let option_rc2 = Rc::clone(&option_rc0);

                buttons.push(|cx: &mut Context| {
                    SelectorButton::new(cx, move |cx| {
                        let cloned_option = Rc::clone(&option_rc0);
                        cx.emit(SelectorEvent::Select(*cloned_option))
                    }, move |cx| {
                        let cloned_option = Rc::clone(&option_rc1);
                        match param_to_label(*cloned_option) {
                            ButtonLabel::Text(name) => {
                                HStack::new(cx, |cx| { Label::new(cx, &name); })
                            }
                            #[allow(unreachable_code, unused_variables)]
                            ButtonLabel::Image(img) => {
                                panic!("Images are not supported in nih_plug_vizia :(");
                                HStack::new(cx, |cx| { Image::new(cx, &img); })
                            }
                        }
                    }).checked(ParamWidgetBase::make_lens(params.clone(), params_to_param, move |param| {
                        let cloned_option = Rc::clone(&option_rc2);
                        *cloned_option == param.value()
                    }));
                })
            }

            Grid::new(2,
                      GridVerticalModifiers {
                          col_between: Pixels(5.0),
                          child_bottom: Auto,
                          child_top: Auto,
                          child_left: Stretch(1.0),
                          child_right: Stretch(1.0),
                      },
                      cx,
                      buttons,
            ).row_between(Pixels(5.0));
        })
    }
}

impl<T> View for Selector<T>
    where T: PartialEq + Enum + 'static + Sequence + Send + Copy
{
    fn element(&self) -> Option<&'static str> {
        Some("param-selector")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|selector_event: &SelectorEvent<T>, _meta| match selector_event {
            SelectorEvent::Select(v) => {
                let value = EnumParam::new("", *v);

                self.param_base.begin_set_parameter(cx);
                self.param_base.set_normalized_value(cx, value.unmodulated_normalized_value());
                self.param_base.end_set_parameter(cx);
            }
        })
    }
}

pub fn get_enum_name<E>(enum_value: E) -> String
    where E: PartialEq + Enum + 'static + Sequence + Send + Copy {
    EnumParam::new("", enum_value).to_string()
}

#[allow(dead_code)]
pub enum ButtonLabel {
    Text(String),
    Image(String),
}

// TODO the only way to copy button functionality and changing the element name
//  seems to be to actually copy the code :(
struct SelectorButton {
    action: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl SelectorButton {
    pub fn new<A, F, V>(cx: &mut Context, action: A, content: F) -> Handle<Self>
        where
            A: 'static + Fn(&mut EventContext),
            F: FnOnce(&mut Context) -> Handle<V>,
            V: 'static + View,
    {
        Self { action: Some(Box::new(action)) }
            .build(cx, move |cx| {
                (content)(cx).hoverable(false);
            })
            .cursor(CursorIcon::Hand)
            .navigable(true)
    }
}

impl View for SelectorButton {
    fn element(&self) -> Option<&'static str> {
        Some("param-button")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::PressDown { .. } => {
                cx.capture();
                cx.focus();
            }

            WindowEvent::Press { .. } => {
                if meta.target == cx.current() {
                    if let Some(callback) = &self.action {
                        (callback)(cx);
                    }
                    cx.release();
                }
            }

            _ => {}
        });
    }
}