use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::RawParamEvent;

pub struct ParamButtonWrapper<F>
    where F: FnMut(&mut EventContext) + 'static
{
    on_event: F,
}

impl<F> ParamButtonWrapper<F>
    where F: FnMut(&mut EventContext) + 'static
{
    pub fn new<C>(cx: &mut Context, content: C, on_event: F) -> Handle<Self>
        where C: FnOnce(&mut Context), {
        Self {
            on_event
        }.build(cx, |cx| {
            (content)(cx)
        })
    }
}


impl<F> View for ParamButtonWrapper<F>
    where F: FnMut(&mut EventContext) + 'static
{
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|param_event: &RawParamEvent, _meta| {
            // Emit a RemoveOscillator event if a parameter inside this wrapper is set
            // This is the case when the user presses the "X" button for an oscillator
            if let RawParamEvent::SetParameterNormalized(_, _) = param_event {
                (self.on_event)(cx);
                // cx.emit(ControlEvent::RemoveOscillator)
            }
        })
    }
}