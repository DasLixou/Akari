use vizia::prelude::*;

#[derive(Lens, Clone)]
pub struct SidebarElement {
    pub text: String,
}

impl SidebarElement {
    pub fn new<'a, T>(cx: &'a mut Context, text: impl Res<T>) -> Handle<'a, Button>
    where
        T: ToString,
    {
        Button::new(
            cx,
            |_| {},
            |cx| {
                Label::new(cx, text)
                    .color(Color::white())
                    .overflow(Overflow::Hidden)
            },
        )
        .size(super::SIDEBAR_ELEMENT_SIZE)
        .max_size(super::SIDEBAR_ELEMENT_SIZE)
        .class("sidebar_element")
    }
}
