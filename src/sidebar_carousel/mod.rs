pub mod item;
pub mod sidebar;

use vizia::prelude::{Wrapper, *};

use crate::AppEvent;

use self::item::{ItemBehaviour, Items, SidebarItem};

#[derive(Clone)]
pub struct BuildClosure(pub fn(&mut Context));

impl BuildClosure {
    #[inline]
    pub fn build(&self, cx: &mut Context) {
        (self.0)(cx)
    }
}

impl Data for BuildClosure {
    fn same(&self, other: &Self) -> bool {
        self.0 as usize == other.0 as usize
    }
}

pub enum SidebarCarouselEvent {
    PressItem(usize),
    ShowMainItems,
    ShowSubItems(Items),
}

#[derive(Lens)]
pub struct SidebarCarousel {
    pub main_items: Vec<SidebarItem>,
    pub items: Vec<SidebarItem>,
    pub selected: usize,
}

impl SidebarCarousel {
    pub fn new(main_items: Items) -> Self {
        Self {
            main_items: main_items.0.clone(),
            items: main_items.0,
            selected: 0,
        }
    }
}

impl Model for SidebarCarousel {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        if let Some(event) = event.take() {
            match event {
                SidebarCarouselEvent::PressItem(index) => {
                    self.selected = index;
                    match &self.items[index].behaviour {
                        ItemBehaviour::Page(closure) => {
                            cx.emit(AppEvent::ChangeContent(closure.clone()))
                        }
                        ItemBehaviour::Nothing => {}
                    }
                }
                SidebarCarouselEvent::ShowMainItems => {
                    self.items = self.main_items.clone();
                }
                SidebarCarouselEvent::ShowSubItems(items) => {
                    self.items = items.0;
                }
            }
        }
    }
}
