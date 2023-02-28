pub mod carousel;
pub mod item;
pub mod sidebar;
pub mod toggle_lens;

use vizia::prelude::{Wrapper, *};

use self::item::{Items, SidebarItem};
use self::toggle_lens::Toggle;

use self::sidebar_carousel_derived_lenses as d;

pub enum SidebarCarouselEvent {
    SelectItem(usize),
    ShowMainItems,
    ShowSubItems(Items),
}

#[derive(Lens)]
pub struct SidebarCarousel {
    pub main_items: Vec<SidebarItem>,
    pub sub_items: Vec<SidebarItem>,
    pub toggle: bool,
    pub selected: usize,
}

impl SidebarCarousel {
    #[allow(non_upper_case_globals)]
    const items: Toggle<Wrapper<d::main_items>, Wrapper<d::sub_items>, Wrapper<d::toggle>> =
        Toggle::new(
            SidebarCarousel::main_items,
            SidebarCarousel::sub_items,
            SidebarCarousel::toggle,
        );

    pub fn new(main_items: Items) -> Self {
        Self {
            main_items: main_items.0,
            sub_items: Vec::with_capacity(0),
            toggle: false,
            selected: 0,
        }
    }
}

impl Model for SidebarCarousel {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        if let Some(event) = event.take() {
            match event {
                SidebarCarouselEvent::SelectItem(index) => {
                    self.selected = index;
                }
                SidebarCarouselEvent::ShowMainItems => {
                    self.toggle = false;
                }
                SidebarCarouselEvent::ShowSubItems(items) => {
                    self.sub_items = items.0;
                    self.toggle = true;
                }
            }
        }
    }
}
