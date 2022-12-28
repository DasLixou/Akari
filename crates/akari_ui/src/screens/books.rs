use dioxus::prelude::*;

pub fn BooksScreen(cx: Scope) -> Element {
    cx.render(rsx! {
        main {
            class: "p-3",
        }
    })
}
