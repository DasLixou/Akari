use dioxus::prelude::*;

use crate::{
    hooks::use_sidebar,
    icons::Icon,
    sidebar::{SidebarData, SidebarElement},
};

pub fn SettingsScreen(cx: Scope) -> Element {
    use_sidebar(&cx, || {
        SidebarData::Custom(&[SidebarElement::Navigator(Icon::CalenderDays, "/")])
    });

    cx.render(rsx! {
        div {}
    })
}
