use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::*;
use log::info;

use crate::{
    hooks::UseSidebar,
    icons::{Icon, OutlineIcon, SolidIcon},
    sidebar::{SidebarData, SidebarElement},
};

// TODO: maybe move and make it modular
fn Applications(cx: Scope) -> Element {
    cx.render(rsx! {
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
    })
}

pub(crate) fn init_sidebar_data(cx: &Scope) {
    cx.provide_context(Rc::new(UseSidebar(RefCell::new(SidebarData::Applications))));
}

pub fn Sidebar<'a>(cx: Scope) -> Element {
    let sidebar_data = cx.use_hook(|_| match cx.consume_context::<Rc<UseSidebar>>() {
        Some(data) => data,
        None => panic!("Couldn't find UseSidebar"),
    }); // TODO: THIS DOESNT CHANGE :C

    cx.render(rsx! {
        ul {
            class: "flex flex-col shadow h-screen min-h-screen w-16 items-center sticky left-0 top-0",
            match *sidebar_data.0.borrow() {
                SidebarData::Applications => rsx! { Applications {} },
                SidebarData::Custom(elements) => rsx! {
                    elements.iter().map(|elem| match elem {
                        SidebarElement::Seperator => rsx!(div { class: "grow" }),
                        SidebarElement::Navigator(icon, path) => rsx!(NavigationSidebarElement {
                            icon: icon.clone(),
                            path: path,
                        }),
                    })
                },
            }
        }
    })
}

#[inline_props]
pub fn SidebarElementBox<'a>(cx: Scope, class: Option<&'a str>, children: Element<'a>) -> Element {
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
            SidebarElementBox {
                class: "{class}",
                icon
            }
        }
    })
}
