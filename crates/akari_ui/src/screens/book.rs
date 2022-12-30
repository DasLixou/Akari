use akari_books::BookFiles;
use dioxus::prelude::*;

use crate::{hooks::use_sidebar, sidebar::SidebarData};

pub fn BookScreen(cx: Scope) -> Element {
    // TODO: make every BookFiles one context
    let files = cx.use_hook(|_| BookFiles::new());

    let route = use_route(&cx);

    let book = route
        .segment("book")
        .expect("Couldn't read book identifier");

    use_sidebar(&cx, || SidebarData::SubPage(&[]));

    let html = files
        .books
        .get_mut(&book.to_owned())
        .unwrap()
        .doc
        .get_current_str()
        .unwrap();

    cx.render(rsx! {
        rsx! {
            div {
                dangerous_inner_html: "{html}"
            }
        }
    })
}
