use nih_plug_vizia::vizia::prelude::*;

// TODO the only way to copy button functionality and changing the element name
//  seems to be to actually copy the code :(
pub struct FakeParamButton {
    action: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl FakeParamButton {
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

impl View for FakeParamButton {
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