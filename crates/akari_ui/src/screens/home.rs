use dioxus::prelude::*;

use crate::{hooks::use_sidebar, sidebar::SidebarData};

pub fn HomeScreen(cx: Scope) -> Element {
    use_sidebar(&cx, || SidebarData::Applications);

    cx.render(rsx! {
        h1 { "Akari" }
    })
}
