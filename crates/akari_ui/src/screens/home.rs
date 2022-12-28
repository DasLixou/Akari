use dioxus::prelude::*;

pub fn HomeScreen(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            class: "p-3",
            h1 { class: "flex justify-center", "Akari | Home" }
        }
    })
}
