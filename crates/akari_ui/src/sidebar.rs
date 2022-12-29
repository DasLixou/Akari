use crate::icons::Icon;

pub enum SidebarData<'a> {
    Applications,
    Custom(&'a [SidebarElement<'a>]),
}

pub enum SidebarElement<'a> {
    Navigator(Icon, &'a str),
    Seperator,
}
