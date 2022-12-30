use dioxus::prelude::*;

use crate::{
    hooks::use_sidebar,
    icons::Icon,
    sidebar::{SidebarData, SidebarElement},
};

pub fn SettingsScreen(cx: Scope) -> Element {
    use_sidebar(&cx, || {
        SidebarData::SubPage(&[
            SidebarElement::Navigator(Icon::Cog8Tooth, "/settings/example"),
            SidebarElement::Navigator(Icon::Cog8Tooth, "/settings/example2"),
        ])
    });

    cx.render(rsx! {
        div {}
    })
}
