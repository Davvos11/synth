use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::{ParamSliderStyle, util};
use nih_plug_vizia::widgets::util::ModifiersExt;

/// When shift+dragging a parameter, one pixel dragged corresponds to this much change in the
/// normalized parameter.
const GRANULAR_DRAG_MULTIPLIER: f32 = 0.1;
const STEP_SIZE: f32 = 0.02;
const STEP_SIZE_FINE: f32 = 0.005;

/// A modified copy of [`ParamSlider`], that uses `on_changing` rather than change [`Param`] types.
#[derive(Lens)]
pub struct FakeParamSlider<L: Lens> {
    lens: L,
    on_changing: Option<Box<dyn Fn(&mut EventContext, f32)>>,
    default_value: f32,

    /// Will be set to `true` when the field gets Alt+Click'ed which will replace the label with a
    /// text box.
    text_input_active: bool,
    /// Will be set to `true` if we're dragging the parameter. Resetting the parameter or entering a
    /// text value should not initiate a drag.
    drag_active: bool,
    /// We keep track of the start coordinate and normalized value when holding down Shift while
    /// dragging for higher precision dragging. This is a `None` value when granular dragging is not
    /// active.
    granular_drag_status: Option<GranularDragStatus>,

    // These fields are set through modifiers:
    /// Whether or not to listen to scroll events for changing the parameter's value in steps.
    use_scroll_wheel: bool,
    /// The number of (fractional) scrolled lines that have not yet been turned into parameter
    /// change events. This is needed to support trackpads with smooth scrolling.
    scrolled_lines: f32,
    /// What style to use for the slider.
    style: ParamSliderStyle,
    /// A specific label to use instead of displaying the parameter's value.
    label_override: Option<String>,
}

enum ParamSliderEvent {
    /// Text input has been cancelled without submitting a new value.
    CancelTextInput,
    /// A new value has been sent by the text input dialog after pressing Enter.
    TextInput(String),
}

// sourceTODO: Vizia's lens derive macro requires this to be marked as pub
#[derive(Debug, Clone, Copy)]
pub struct GranularDragStatus {
    /// The mouse's X-coordinate when the granular drag was started.
    pub starting_x_coordinate: f32,
    /// The normalized value when the granular drag was started.
    pub starting_value: f32,
}

impl<L> FakeParamSlider<L>
    where L: Lens<Target=f32>
{
    /// Creates a new [`FakeParamSlider`] for the given lens value. Parameter changes are
    /// handled by the `on_changing` handle extension.
    ///
    /// See [`FakeParamSliderExt`] for additional options.
    pub fn new(
        cx: &mut Context,
        lens: L,
        default_value: f32,
    ) -> Handle<Self> {
        // We'll visualize the difference between the current value and the default value if the
        // default value lies somewhere in the middle and the parameter is continuous. Otherwise
        // this approach looks a bit jarring.
        Self {
            lens: lens.clone(),
            on_changing: None,
            default_value,

            text_input_active: false,
            drag_active: false,
            granular_drag_status: None,

            use_scroll_wheel: true,
            scrolled_lines: 0.0,
            style: ParamSliderStyle::Centered,
            label_override: None,
        }
            .build(
                cx,
                |cx| {
                    Binding::new(cx, FakeParamSlider::<L>::style, move |cx, style| {
                        let style = style.get(cx);

                        // Needs to be moved into the below closures, and it can't be `Copy`
                        let lens = lens.clone();

                        // TODO implement format settings
                        let display_value_lens = lens.clone().map(|f| {
                            format!("{:.2}", f)
                        });
                        // The resulting tuple `(start_t, delta)` corresponds to the start and the
                        // signed width of the bar. `start_t` is in `[0, 1]`, and `delta` is in
                        // `[-1, 1]`.
                        let fill_start_delta_lens = {
                            let lens = lens.clone();
                            lens.map(move |current_value| {
                                Self::compute_fill_start_delta(
                                    style,
                                    *current_value,
                                    0.0, // TODO implement zero value
                                )
                            })
                        };

                        // Only draw the text input widget when it gets focussed. Otherwise, overlay the
                        // label with the slider. Creating the textbox based on
                        // `ParamSliderInternal::text_input_active` lets us focus the textbox when it gets
                        // created.
                        Binding::new(
                            cx,
                            FakeParamSlider::<L>::text_input_active,
                            move |cx, text_input_active| {
                                if text_input_active.get(cx) {
                                    Self::text_input_view(cx, display_value_lens.clone());
                                } else {
                                    // All of this data needs to be moved into the `ZStack` closure, and
                                    // the `Map` lens combinator isn't `Copy`
                                    let display_value_lens = display_value_lens.clone();
                                    let fill_start_delta_lens = fill_start_delta_lens.clone();

                                    ZStack::new(cx, move |cx| {
                                        Self::slider_fill_view(
                                            cx,
                                            fill_start_delta_lens,
                                        );
                                        Self::slider_label_view(
                                            cx,
                                            display_value_lens,
                                            FakeParamSlider::<L>::label_override,
                                        );
                                    })
                                        .hoverable(false);
                                }
                            },
                        );
                    },
                    );
                },
            )
    }

    /// Create a text input that's shown in place of the slider.
    fn text_input_view(cx: &mut Context, display_value_lens: impl Lens<Target=String>) {
        Textbox::new(cx, display_value_lens)
            .class("value-entry")
            .on_submit(|cx, string, success| {
                if success {
                    cx.emit(ParamSliderEvent::TextInput(string))
                } else {
                    cx.emit(ParamSliderEvent::CancelTextInput);
                }
            })
            .on_build(|cx| {
                cx.emit(TextEvent::StartEdit);
                cx.emit(TextEvent::SelectAll);
            })
            // `.child_space(Stretch(1.0))` no longer works
            .class("align_center")
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .height(Stretch(1.0))
            .width(Stretch(1.0));
    }

    /// Create the fill part of the slider.
    fn slider_fill_view(
        cx: &mut Context,
        fill_start_delta_lens: impl Lens<Target=(f32, f32)>,
    ) {
        // The filled bar portion. This can be visualized in a couple different ways depending on
        // the current style property. See [`ParamSliderStyle`].
        Element::new(cx)
            .class("fill")
            .height(Stretch(1.0))
            .left(
                fill_start_delta_lens
                    .clone()
                    .map(|(start_t, _)| Percentage(start_t * 100.0)),
            )
            .width(fill_start_delta_lens.map(|(_, delta)| Percentage(delta * 100.0)))
            // Hovering is handled on the param slider as a whole, this
            // should not affect that
            .hoverable(false);
    }

    /// Create the text part of the slider. Shown on top of the fill using a `ZStack`.
    fn slider_label_view(
        cx: &mut Context,
        display_value_lens: impl Lens<Target=String>,
        label_override_lens: impl Lens<Target=Option<String>>,
    ) {
        // Display the current value
        // sourceTODO: Do the same thing as in the iced widget where we draw the
        //       text overlapping the fill area slightly differently. We can
        //       set the cip region directly in vizia.

        Binding::new(cx, label_override_lens, move |cx, label_override_lens| {
            // If the label override is set then we'll use that. If not, the parameter's
            // current display value (before modulation) is used.
            match label_override_lens.get(cx) {
                Some(label_override) => Label::new(cx, &label_override),
                None => Label::new(cx, display_value_lens.clone()),
            }
                .class("value")
                .class("value--single")
                .child_space(Stretch(1.0))
                .height(Stretch(1.0))
                .width(Stretch(1.0))
                .hoverable(false);
        });
    }

    /// Calculate the start position and width of the slider's fill region based on the selected
    /// style, the parameter's current value, and the parameter's step sizes. The resulting tuple
    /// `(start_t, delta)` corresponds to the start and the signed width of the bar. `start_t` is in
    /// `[0, 1]`, and `delta` is in `[-1, 1]`.
    fn compute_fill_start_delta(
        style: ParamSliderStyle,
        current_value: f32,
        zero_value: f32,
    ) -> (f32, f32) {
        let draw_fill_from_default = matches!(style, ParamSliderStyle::Centered)
            && (0.45..=0.55).contains(&zero_value);

        match style {
            ParamSliderStyle::Centered if draw_fill_from_default => {
                let delta = (zero_value - current_value).abs();

                // Don't draw the filled portion at all if it could have been a
                // rounding error since those slivers just look weird
                (
                    zero_value.min(current_value),
                    if delta >= 1e-3 { delta } else { 0.0 },
                )
            }
            ParamSliderStyle::Centered | ParamSliderStyle::FromLeft => (0.0, current_value),
            _ => { panic!("Steps are unsupported for FakeParamSlider") }
        }
    }

    /// `self.param_base.set_normalized_value()`, but resulting from a mouse drag. When using the
    /// 'even' stepped slider styles from [`ParamSliderStyle`] this will remap the normalized range
    /// to match up with the fill value display. This still needs to be wrapped in a parameter
    /// automation gesture.
    fn set_normalized_value_drag(&self, cx: &mut EventContext, normalized_value: f32) {
        if let Some(callback) = &self.on_changing {
            (callback)(cx, normalized_value);
        }
    }
}

impl<L> View for FakeParamSlider<L>
    where L: Lens<Target=f32>
{
    fn element(&self) -> Option<&'static str> {
        Some("param-slider")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|param_slider_event, meta| match param_slider_event {
            ParamSliderEvent::CancelTextInput => {
                self.text_input_active = false;
                cx.set_active(false);

                meta.consume();
            }
            ParamSliderEvent::TextInput(string) => {
                if let Ok(value) = string.parse::<f32>() {
                    if let Some(callback) = &self.on_changing {
                        (callback)(cx, value);
                    }
                }


                self.text_input_active = false;

                meta.consume();
            }
        });

        event.map(|window_event, meta| match window_event {
            // Vizia always captures the third mouse click as a triple click. Treating that triple
            // click as a regular mouse button makes double click followed by another drag work as
            // expected, instead of requiring a delay or an additional click. Double double click
            // still won't work.
            WindowEvent::MouseDown(MouseButton::Left)
            | WindowEvent::MouseTripleClick(MouseButton::Left) => {
                if cx.modifiers.alt() {
                    // ALt+Click brings up a text entry dialog
                    self.text_input_active = true;
                    cx.set_active(true);
                } else if cx.modifiers.command() {
                    // Ctrl+Click, double click, and right clicks should reset the parameter instead
                    // of initiating a drag operation
                    if let Some(callback) = &self.on_changing {
                        (callback)(cx, self.default_value);
                    }
                } else {
                    self.drag_active = true;
                    cx.capture();
                    // NOTE: Otherwise we don't get key up events
                    cx.focus();
                    cx.set_active(true);

                    // When holding down shift while clicking on a parameter we want to granuarly
                    // edit the parameter without jumping to a new value
                    if cx.modifiers.shift() {
                        self.granular_drag_status = Some(GranularDragStatus {
                            starting_x_coordinate: cx.mouse.cursorx,
                            starting_value: self.lens.get(cx),
                        });
                    } else {
                        self.granular_drag_status = None;
                        self.set_normalized_value_drag(
                            cx,
                            util::remap_current_entity_x_coordinate(cx, cx.mouse.cursorx),
                        );
                    }
                }

                meta.consume();
            }
            WindowEvent::MouseDoubleClick(MouseButton::Left)
            | WindowEvent::MouseDown(MouseButton::Right)
            | WindowEvent::MouseDoubleClick(MouseButton::Right)
            | WindowEvent::MouseTripleClick(MouseButton::Right) => {
                // Ctrl+Click, double click, and right clicks should reset the parameter instead of
                // initiating a drag operation
                if let Some(callback) = &self.on_changing {
                    (callback)(cx, self.default_value);
                }

                meta.consume();
            }
            WindowEvent::MouseUp(MouseButton::Left) => {
                if self.drag_active {
                    self.drag_active = false;
                    cx.release();
                    cx.set_active(false);

                    meta.consume();
                }
            }
            WindowEvent::MouseMove(x, _y) => {
                if self.drag_active {
                    // If shift is being held then the drag should be more granular instead of
                    // absolute
                    if cx.modifiers.shift() {
                        let granular_drag_status =
                            *self
                                .granular_drag_status
                                .get_or_insert_with(|| GranularDragStatus {
                                    starting_x_coordinate: *x,
                                    starting_value: self.lens.get(cx),
                                });

                        // These positions should be compensated for the DPI scale so it remains
                        // consistent
                        let start_x =
                            util::remap_current_entity_x_t(cx, granular_drag_status.starting_value);
                        let delta_x = ((*x - granular_drag_status.starting_x_coordinate)
                            * GRANULAR_DRAG_MULTIPLIER)
                            * cx.style.dpi_factor as f32;

                        self.set_normalized_value_drag(
                            cx,
                            util::remap_current_entity_x_coordinate(cx, start_x + delta_x),
                        );
                    } else {
                        self.granular_drag_status = None;

                        self.set_normalized_value_drag(
                            cx,
                            util::remap_current_entity_x_coordinate(cx, *x),
                        );
                    }
                }
            }
            WindowEvent::KeyUp(_, Some(Key::Shift)) => {
                // If this happens while dragging, snap back to reality uh I mean the current screen
                // position
                if self.drag_active && self.granular_drag_status.is_some() {
                    self.granular_drag_status = None;
                    if let Some(callback) = &self.on_changing {
                        (callback)(cx, util::remap_current_entity_x_coordinate(cx, cx.mouse.cursorx));
                    }
                }
            }
            WindowEvent::MouseScroll(_scroll_x, scroll_y) if self.use_scroll_wheel => {
                // With a regular scroll wheel `scroll_y` will only ever be -1 or 1, but with smooth
                // scrolling trackpads being a thing `scroll_y` could be anything.
                self.scrolled_lines += scroll_y;

                if self.scrolled_lines.abs() >= 1.0 {
                    let use_finer_steps = cx.modifiers.shift();

                                        let mut current_value = self.lens.get(cx);

                    while self.scrolled_lines >= 1.0 {
                        // TODO implement min and max value
                        current_value += if use_finer_steps { STEP_SIZE_FINE } else { STEP_SIZE };
                        if let Some(callback) = &self.on_changing {
                            (callback)(cx, current_value);
                        }
                        self.scrolled_lines -= 1.0;
                    }

                    while self.scrolled_lines <= -1.0 {
                        current_value -= if use_finer_steps { STEP_SIZE_FINE } else { STEP_SIZE };
                        if let Some(callback) = &self.on_changing {
                            (callback)(cx, current_value);
                        }
                        self.scrolled_lines += 1.0;
                    }

                }

                meta.consume();
            }
            _ => {}
        });
    }
}

/// Extension methods for [`FakeParamSlider`] handles.
pub trait FakeParamSliderExt {
    /// Don't respond to scroll wheel events. Useful when this slider is used as part of a scrolling
    /// view.
    fn disable_scroll_wheel(self) -> Self;

    /// Change how the [`FakeParamSlider`] visualizes the current value.
    fn set_style(self, style: ParamSliderStyle) -> Self;

    /// Manually set a fixed label for the slider instead of displaying the current value. This is
    /// currently not reactive.
    fn with_label(self, value: impl Into<String>) -> Self;
}

impl<L> FakeParamSliderExt for Handle<'_, FakeParamSlider<L>>
    where L: Lens<Target=f32>
{
    fn disable_scroll_wheel(self) -> Self {
        self.modify(|param_slider: &mut FakeParamSlider<L>| param_slider.use_scroll_wheel = false)
    }

    fn set_style(self, style: ParamSliderStyle) -> Self {
        self.modify(|param_slider: &mut FakeParamSlider<L>| param_slider.style = style)
    }

    fn with_label(self, value: impl Into<String>) -> Self {
        self.modify(|param_slider: &mut FakeParamSlider<L>| {
            param_slider.label_override = Some(value.into())
        })
    }
}

pub trait SliderHandle {
    fn on_changing<F>(self, callback: F) -> Self
        where F: 'static + Fn(&mut EventContext, f32),;
}

impl<L: Lens> SliderHandle for Handle<'_, FakeParamSlider<L>>
{
    fn on_changing<F>(self, callback: F) -> Self
        where F: 'static + Fn(&mut EventContext, f32),
    {
        self.modify(|slider| slider.on_changing = Some(Box::new(callback)))
    }
}