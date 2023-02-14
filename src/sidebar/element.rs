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
        .border_width(Units::Pixels(0.))
        .border_radius(Units::Pixels(0.))
        .background_color(Color::black())
        .size(super::SIDEBAR_ELEMENT_SIZE)
        .max_size(super::SIDEBAR_ELEMENT_SIZE)
        .overflow(Overflow::Hidden)
    }
}
