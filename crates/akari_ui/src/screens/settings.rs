use dioxus::prelude::*;

use crate::components::sidebar::MainSidebar;

pub fn SettingsScreen(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            class: "flex select-none",
            MainSidebar {}
        }
    })
}
