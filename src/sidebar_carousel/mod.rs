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

impl PartialEq for BuildClosure {
    fn eq(&self, other: &Self) -> bool {
        self.0 as usize == other.0 as usize
    }
}

impl Data for BuildClosure {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

#[derive(Clone)]
pub struct EventClosure(pub fn(&mut EventContext));

impl EventClosure {
    #[inline]
    pub fn build(&self, cx: &mut EventContext) {
        (self.0)(cx)
    }
}

impl PartialEq for EventClosure {
    fn eq(&self, other: &Self) -> bool {
        self.0 as usize == other.0 as usize
    }
}

impl Data for EventClosure {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
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
    pub sub_items: Vec<SidebarItem>,
    pub items: Vec<SidebarItem>,
    pub selected: usize,
}

impl SidebarCarousel {
    pub fn new(main_items: Items) -> Self {
        Self {
            main_items: main_items.0.clone(),
            sub_items: vec![],
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
                    match &self.items[index] {
                        SidebarItem::Button(_, behaviour) => match behaviour {
                            ItemBehaviour::Page(closure) => {
                                cx.emit(AppEvent::ChangeContent(closure.clone()));
                                self.items = self.sub_items.clone();
                            }
                            ItemBehaviour::Action(closure) => {
                                closure.build(cx);
                            }
                            ItemBehaviour::ShowMainBar => {
                                self.items = self.main_items.clone();
                            }
                            ItemBehaviour::Nothing => {}
                        },
                        SidebarItem::Spacer => {}
                    }
                }
                SidebarCarouselEvent::ShowMainItems => {
                    self.items = self.main_items.clone();
                }
                SidebarCarouselEvent::ShowSubItems(items) => {
                    self.sub_items = items.0;
                    self.items = self.sub_items.clone();
                }
            }
        }
    }
}
