use vizia::prelude::*;

use super::{button::SidebarButton, SidebarCarousel, SidebarCarouselEvent, SidebarItem};

pub const SIDEBAR_ELEMENT_SIZE: Units = Pixels(60.);

pub struct Sidebar;

impl Sidebar {
    pub fn new(cx: &mut Context) -> Handle<'_, Self> {
        Sidebar
            .build(cx, |cx| {
                cx.add_stylesheet("src/sidebar_carousel/style.css").unwrap();

                List::new(cx, SidebarCarousel::items, |cx, index, item| {
                    SidebarButton::new(
                        cx,
                        item.then(SidebarItem::text),
                        move |cx| cx.emit(SidebarCarouselEvent::SelectItem(index)),
                        SidebarCarousel::selected.map(move |i| index == *i),
                    );
                })
                .width(SIDEBAR_ELEMENT_SIZE)
                .class("sidebar");
            })
            .width(SIDEBAR_ELEMENT_SIZE)
    }
}

impl View for Sidebar {}
