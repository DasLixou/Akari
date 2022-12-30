use crate::icons::Icon;

#[derive(PartialEq, Clone)]
pub enum SidebarData<'a> {
    Applications,
    SubPage(&'a [SidebarElement<'a>]),
}

#[derive(PartialEq, Clone)]
pub enum SidebarElement<'a> {
    Navigator(Icon, &'a str),
    Seperator,
}
