use vizia::{
    prelude::{Context, Handle},
    state::{Binding, LensExt},
    view::View,
};

use super::SidebarCarousel;

pub struct Carousel;

impl Carousel {
    pub fn new(cx: &mut Context) -> Handle<'_, Self> {
        Carousel.build(cx, |cx| {
            Binding::new(cx, SidebarCarousel::selected, |cx, lens| {
                let selected = lens.get(cx);
                (SidebarCarousel::items.get(cx)[selected].content)(cx);
            })
        })
    }
}

impl View for Carousel {}
