use dioxus::prelude::*;

use crate::{hooks::use_sidebar, sidebar::SidebarData};

pub fn BookScreen(cx: Scope) -> Element {
    let route = use_route(&cx);

    let book = match route.segment("book") {
        Some(val) => val,
        None => "An unknown error occured",
    };

    use_sidebar(&cx, || SidebarData::Applications);

    cx.render(rsx! {
        h1 { "Reading {book}" }
    })
}
