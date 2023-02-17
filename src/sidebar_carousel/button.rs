use vizia::prelude::*;

use super::sidebar::SIDEBAR_ELEMENT_SIZE;

#[derive(Lens)]
pub struct SidebarButton {
    action: Option<Box<dyn Fn(&mut EventContext)>>,
}

impl SidebarButton {
    pub fn new<'a, T, A>(
        cx: &'a mut Context,
        text: impl Res<T>,
        action: A,
        active: impl Res<bool>,
    ) -> Handle<Self>
    where
        T: ToString,
        A: 'static + Fn(&mut EventContext),
    {
        Self {
            action: Some(Box::new(action)),
        }
        .build(cx, move |cx| {
            Label::new(cx, text)
                .overflow(Overflow::Hidden)
                .hoverable(false);
        })
        .cursor(CursorIcon::Hand)
        .navigable(true)
        .size(SIDEBAR_ELEMENT_SIZE)
        .max_size(SIDEBAR_ELEMENT_SIZE)
        .checked(active)
    }
}

impl View for SidebarButton {
    fn element(&self) -> Option<&'static str> {
        Some("sidebar_button")
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
