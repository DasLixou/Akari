use dioxus::prelude::*;
use log::info;

use crate::icons::{Icon, OutlineIcon, SolidIcon};

pub fn MainSidebar(cx: Scope) -> Element {
    cx.render(rsx! {
        Sidebar {
            NavigationSidebarElement {
                icon: Icon::DocumentPlus,
                path: "/",
            },
            NavigationSidebarElement {
                icon: Icon::PencilSquare,
                path: "/scribe",
            }
            NavigationSidebarElement {
                icon: Icon::BookOpen,
                path: "/books",
            }
            NavigationSidebarElement {
                icon: Icon::CalenderDays,
                path: "/calender",
            }
            div {
                class: "grow"
            }
            NavigationSidebarElement {
                icon: Icon::Cog8Tooth,
                path: "/settings",
            }
        }
    })
}

#[inline_props]
pub fn Sidebar<'a>(cx: Scope, children: Element<'a>) -> Element {
    cx.render(rsx! {
        ul {
            class: "flex flex-col shadow h-screen min-h-screen w-16 items-center sticky left-0 top-0",
            children
        }
    })
}

#[inline_props]
pub fn SidebarElement<'a>(cx: Scope, class: Option<&'a str>, children: Element<'a>) -> Element {
    let class = class.unwrap_or_default();
    cx.render(rsx! {
        li {
            button {
                class: "flex w-16 h-16 p-3 items-center justify-center overflow-hidden {class}",
                children
            }
        }
    })
}

#[inline_props]
pub fn NavigationSidebarElement<'a>(cx: Scope, icon: Icon, path: &'a str) -> Element {
    let route = use_route(&cx);
    let current_path = route.url().path();
    let active = current_path == *path;

    info!("{} : {} : {}", current_path, active, path);

    let class = match active {
        true => "bg-black text-white",
        false => "bg-white text-black",
    };
    let icon = match active {
        true => rsx! {
            SolidIcon { icon: icon.clone() }
        },
        false => rsx! {
            OutlineIcon { icon: icon.clone() }
        },
    };

    cx.render(rsx! {
        Link {
            to: path,
            SidebarElement {
                class: "{class}",
                icon
            }
        }
    })
}
