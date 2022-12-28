use dioxus::prelude::*;

pub fn HomeScreen(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            h1 { class: "flex justify-center", "Akari | Home" }
        }
    })
}
