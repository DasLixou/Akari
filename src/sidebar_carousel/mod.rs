pub mod button;
pub mod carousel;
pub mod sidebar;

use vizia::prelude::*;

pub enum SidebarCarouselEvent {
    SelectItem(usize),
}

#[derive(Lens, Clone)]
pub struct SidebarItem {
    pub text: String,
    pub content: fn(&mut Context),
}

#[derive(Lens)]
pub struct SidebarCarousel {
    pub items: Vec<SidebarItem>,
    pub selected: usize,
}

impl SidebarCarousel {
    pub fn new(items: Vec<SidebarItem>) -> Self {
        Self { items, selected: 0 }
    }
}

impl Model for SidebarCarousel {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|sidebar_event, _| match sidebar_event {
            SidebarCarouselEvent::SelectItem(index) => {
                self.selected = *index;
            }
        })
    }
}
