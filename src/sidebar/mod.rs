pub mod element;

use vizia::prelude::*;

use self::element::SidebarElement;

pub const SIDEBAR_ELEMENT_SIZE: Units = Pixels(60.);

#[derive(Lens)]
pub struct Sidebar {
    pub sidebar_elements: Vec<SidebarElement>,
}

impl Model for Sidebar {}

impl Sidebar {
    pub fn new<'a>(
        cx: &'a mut Context,
    ) -> Handle<List<sidebar_derived_lenses::sidebar_elements, SidebarElement>> {
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
        }
        .build(cx);

        List::new(cx, Sidebar::sidebar_elements, |cx, _index, item| {
            SidebarElement::new(cx, item.then(SidebarElement::text));
        })
        .width(SIDEBAR_ELEMENT_SIZE)
        .class("sidebar")
    }
}
