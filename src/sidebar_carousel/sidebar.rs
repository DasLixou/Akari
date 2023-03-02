use vizia::prelude::*;

use crate::closures::InitContext;

use super::{SidebarCarousel, SidebarCarouselEvent, SidebarItem};

pub const SIDEBAR_ELEMENT_SIZE: Units = Pixels(60.);

pub struct Sidebar;

impl Sidebar {
    pub fn new(cx: &mut Context) -> Handle<'_, Self> {
        Sidebar
            .build(cx, |cx| {
                cx.add_stylesheet("src/sidebar_carousel/style.css").unwrap();

                List::new(cx, SidebarCarousel::items, |cx, index, lens| {
                    Binding::new(cx, lens, move |cx, lens| {
                        let item = lens.get(cx);
                        match item {
                            SidebarItem::Button(text, _, init_closure) => {
                                Button::new(
                                    cx,
                                    move |cx| cx.emit(SidebarCarouselEvent::PressItem(index)),
                                    |cx| {
                                        init_closure.run(&mut InitContext { cx, index });
                                        Label::new(cx, &text)
                                    },
                                )
                                .class("sidebar_button");
                            }
                            SidebarItem::Spacer => {
                                Element::new(cx)
                                    .class("sidebar_element")
                                    .height(Stretch(1.));
                            }
                        }
                    });
                });
            })
            .class("sidebar")
            .width(SIDEBAR_ELEMENT_SIZE)
    }
}

impl View for Sidebar {}
