use dioxus::prelude::*;

pub fn HomeScreen(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Akari" }
    })
}
