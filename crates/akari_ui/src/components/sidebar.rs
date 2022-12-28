use dioxus::prelude::*;

use crate::icons::{cog_8_tooth::Cog8ToothIcon, document_plus::DocumentPlusIcon};

pub fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx! {
        ul {
            class: "flex flex-col shadow h-screen min-h-screen w-16 items-center",
            SidebarElement {
                "Akari"
            },
            SidebarElement {
                DocumentPlusIcon {}
            }
            div {
                class: "grow"
            }
            SidebarElement {
                Cog8ToothIcon {}
            }
        }
    })
}

#[derive(Props)]
pub struct SidebarElementProps<'a> {
    children: Element<'a>,
}

pub fn SidebarElement<'a>(cx: Scope<'a, SidebarElementProps<'a>>) -> Element {
    cx.render(rsx! {
        li {
            class: "flex w-16 h-16 p-3 items-center justify-center overflow-hidden",
            &cx.props.children
        }
    })
}
