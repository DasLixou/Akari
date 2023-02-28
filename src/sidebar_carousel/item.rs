use vizia::prelude::*;

use super::BuildClosure;

#[derive(Lens, Clone)]
pub struct SidebarItem {
    pub text: String,
    pub behaviour: ItemBehaviour,
}

#[derive(Clone)]
pub enum ItemBehaviour {
    Page(BuildClosure),
    Nothing, // TODO: remove that
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
