pub mod element;

use vizia::prelude::*;

use self::element::SidebarButton;

pub const SIDEBAR_ELEMENT_SIZE: Units = Pixels(60.);

pub enum SidebarEvent {
    SelectItem(usize),
}

#[derive(Lens, Clone)]
pub struct SidebarItem {
    pub text: String,
}

#[derive(Lens)]
pub struct Sidebar {
    pub sidebar_items: Vec<SidebarItem>,
    pub selected: usize,
}

impl Model for Sidebar {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|sidebar_event, _| match sidebar_event {
            SidebarEvent::SelectItem(index) => {
                self.selected = *index;
            }
        })
    }
}

pub fn create_sidebar<'a>(cx: &'a mut Context) -> Handle<impl View> {
    cx.add_stylesheet("src/sidebar/style.css").unwrap();

    Sidebar {
        sidebar_items: vec![
            SidebarItem {
                text: "Akari".into(),
            },
            SidebarItem {
                text: "Scribe".into(),
            },
            SidebarItem {
                text: "Books".into(),
            },
            SidebarItem {
                text: "Calendar".into(),
            },
            SidebarItem {
                text: "Settings".into(),
            },
        ],
        selected: 0,
    }
    .build(cx);

    List::new(cx, Sidebar::sidebar_items, |cx, index, item| {
        SidebarButton::new(
            cx,
            item.then(SidebarItem::text),
            move |cx| cx.emit(SidebarEvent::SelectItem(index)),
            Sidebar::selected.map(move |i| index == *i),
        );
    })
    .width(SIDEBAR_ELEMENT_SIZE)
    .class("sidebar")
}
