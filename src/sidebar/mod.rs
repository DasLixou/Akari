pub mod element;

use vizia::prelude::*;

use self::element::SidebarButton;

pub const SIDEBAR_ELEMENT_SIZE: Units = Pixels(60.);

#[derive(Lens, Clone)]
pub struct SidebarElement {
    pub text: String,
}

#[derive(Lens)]
pub struct Sidebar {
    pub sidebar_elements: Vec<SidebarElement>,
    pub selected: usize,
}

impl Model for Sidebar {}

pub fn create_sidebar<'a>(cx: &'a mut Context) -> Handle<impl View> {
    cx.add_stylesheet("src/sidebar/style.css").unwrap();

    Sidebar {
        sidebar_elements: vec![
            SidebarElement {
                text: "Akari".into(),
            },
            SidebarElement {
                text: "Scribe".into(),
            },
            SidebarElement {
                text: "Books".into(),
            },
            SidebarElement {
                text: "Calendar".into(),
            },
            SidebarElement {
                text: "Settings".into(),
            },
        ],
        selected: 0,
    }
    .build(cx);

    List::new(cx, Sidebar::sidebar_elements, |cx, index, item| {
        SidebarButton::new(
            cx,
            item.then(SidebarElement::text),
            |_| {},
            Sidebar::selected.map(move |i| index == *i),
        );
    })
    .width(SIDEBAR_ELEMENT_SIZE)
    .class("sidebar")
}
