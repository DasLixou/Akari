use vizia::prelude::*;

#[derive(Lens, Clone)]
pub struct SidebarItem {
    pub text: String,
    pub behaviour: ItemBehaviour,
}

#[derive(Clone)]
pub enum ItemBehaviour {
    Page(fn(&mut Context)),
    Nothing, // TODO: remove that
}

impl ItemBehaviour {
    pub fn action(&self, cx: &mut Context) {
        match self {
            ItemBehaviour::Page(page) => {
                (page)(cx);
            }
            ItemBehaviour::Nothing => {}
        }
    }
}

pub struct Items(pub Vec<SidebarItem>);

#[macro_export]
macro_rules! items {
    () => (
        $crate::sidebar_carousel::item::Items(vec![SidebarItem {
            text: "Akari".into(),
            behaviour: $crate::sidebar_carousel::item::ItemBehaviour::Nothing,
        }])
    );
    ($($x:expr),+ $(,)?) => (
        $crate::sidebar_carousel::item::Items(vec![SidebarItem {
            text: "Akari".into(),
            behaviour: $crate::sidebar_carousel::item::ItemBehaviour::Nothing,
        }, $($x),+])
    );
}
