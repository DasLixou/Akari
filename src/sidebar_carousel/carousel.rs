use vizia::{
    prelude::{Context, EmitContext, Handle},
    state::{Binding, LensExt},
    view::View,
};

use super::{SidebarCarousel, SidebarCarouselEvent};

pub struct Carousel;

impl Carousel {
    pub fn new(cx: &mut Context) -> Handle<'_, Self> {
        Carousel.build(cx, |cx| {
            Binding::new(cx, SidebarCarousel::selected, |cx, lens| {
                let selected = lens.get(cx);
                if selected == 0 {
                    cx.emit(SidebarCarouselEvent::ShowMainItems)
                } else {
                    (SidebarCarousel::items.get(cx)[selected - 1].content)(cx);
                }
            })
        })
    }
}

impl View for Carousel {}
