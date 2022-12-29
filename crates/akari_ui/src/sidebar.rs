use crate::icons::Icon;

#[derive(Debug)]
pub enum SidebarData<'a> {
    Applications,
    Custom(&'a [SidebarElement<'a>]),
}

#[derive(Debug)]
pub enum SidebarElement<'a> {
    Navigator(Icon, &'a str),
    Seperator,
}
