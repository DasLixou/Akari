use vizia::prelude::*;

#[derive(Lens, Clone)]
pub struct SidebarItem {
    pub text: String,
    pub content: fn(&mut Context),
}

pub struct Items(pub Vec<SidebarItem>);

#[macro_export]
macro_rules! items {
    () => (
        $crate::sidebar_carousel::item::Items(vec![SidebarItem {
            text: "Akari".into(),
            content: |_| {},
        }])
    );
    ($($x:expr),+ $(,)?) => (
        $crate::sidebar_carousel::item::Items(vec![SidebarItem {
            text: "Akari".into(),
            content: |_| {},
        }, $($x),+])
    );
}
