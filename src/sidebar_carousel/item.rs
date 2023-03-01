use vizia::prelude::Data;

use crate::closures::{BuildClosure, EventClosure};

#[derive(Clone, PartialEq)]
pub enum SidebarItem {
    Button(String, ItemBehaviour),
    Spacer,
}

impl Data for SidebarItem {
    fn same(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

#[derive(Clone, PartialEq)]
pub enum ItemBehaviour {
    Page(BuildClosure),
    Action(EventClosure),
    ShowMainBar,
    Nothing, // TODO: remove that
}

pub struct Items(pub Vec<SidebarItem>);

#[macro_export]
macro_rules! items {
    ($($x:expr),* $(,)?) => (
        $crate::sidebar_carousel::item::Items(vec![SidebarItem::Button(
            "Akari".into(),
            $crate::sidebar_carousel::item::ItemBehaviour::ShowMainBar,
        ), $($x),+])
    );
}
