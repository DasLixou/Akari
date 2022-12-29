use dioxus::prelude::*;

use crate::components::sidebar::MainSidebar;

pub fn ScribeScreen(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            class: "flex",
            MainSidebar {},
            h1 { "Akari" }
        }
    })
}
