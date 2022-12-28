use dioxus::prelude::*;

use crate::icons::{Icon, OutlineIcon, SolidIcon};

pub fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx! {
        ul {
            class: "flex flex-col shadow h-screen min-h-screen w-16 items-center",
            li {
                class: "flex w-16 h-16 p-3 items-center justify-center overflow-hidden",
                "Akari"
            },
            SidebarElement {
                icon: Icon::PencilSquare,
                active: false,
            }
            SidebarElement {
                icon: Icon::BookOpen,
                active: true,
            }
            SidebarElement {
                icon: Icon::CalenderDays,
                active: false,
            }
            div {
                class: "grow"
            }
            SidebarElement {
                icon: Icon::Cog8Tooth,
                active: false,
            }
        }
    })
}

#[derive(Props, PartialEq)]
pub struct SidebarElementProps {
    active: bool,
    icon: Icon,
}

pub fn SidebarElement(cx: Scope<SidebarElementProps>) -> Element {
    let class = match cx.props.active {
        true => {
            "flex w-16 h-16 p-3 items-center justify-center overflow-hidden bg-black text-white"
        }
        false => {
            "flex w-16 h-16 p-3 items-center justify-center overflow-hidden bg-white text-black"
        }
    };
    let icon = cx.props.icon.clone();
    let icon = match cx.props.active {
        true => rsx! {
            SolidIcon { icon: icon }
        },
        false => rsx! {
            OutlineIcon { icon: icon }
        },
    };
    cx.render(rsx! {
        li {
            class: "{class}",
            icon
        }
    })
}
