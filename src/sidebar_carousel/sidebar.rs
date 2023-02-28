use vizia::prelude::*;

use super::{SidebarCarousel, SidebarCarouselEvent, SidebarItem};

pub const SIDEBAR_ELEMENT_SIZE: Units = Pixels(60.);

pub struct Sidebar;

impl Sidebar {
    pub fn new(cx: &mut Context) -> Handle<'_, Self> {
        Sidebar
            .build(cx, |cx| {
                cx.add_stylesheet("src/sidebar_carousel/style.css").unwrap();

                Button::new(
                    cx,
                    move |cx| cx.emit(SidebarCarouselEvent::SelectItem(0)),
                    |cx| Label::new(cx, "Akari"),
                )
                .class("sidebar_button")
                .checked(SidebarCarousel::selected.map(move |i| *i == 0));

                List::new(cx, SidebarCarousel::items, |cx, mut index, item| {
                    index = index + 1; // Akari = 0
                    Button::new(
                        cx,
                        move |cx| cx.emit(SidebarCarouselEvent::SelectItem(index)),
                        |cx| Label::new(cx, item.then(SidebarItem::text)),
                    )
                    .class("sidebar_button")
                    .checked(SidebarCarousel::selected.map(move |i| *i == index));
                });
            })
            .class("sidebar")
            .width(SIDEBAR_ELEMENT_SIZE)
    }
}

impl View for Sidebar {}
