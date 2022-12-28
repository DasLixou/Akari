use dioxus::prelude::*;

pub fn CalenderScreen(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            class: "p-3",
        }
    })
}
