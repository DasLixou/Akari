use dioxus::prelude::*;
use log::info;

use crate::{
    hooks::UseSidebar,
    icons::{Icon, OutlineIcon, SolidIcon},
    sidebar::{SidebarData, SidebarElement},
};

// TODO: maybe move and make it modular
const APPLICATION_SIDEBAR: &'static [SidebarElement<'static>] = &[
    SidebarElement::Navigator(Icon::DocumentPlus, "/"),
    SidebarElement::Navigator(Icon::PencilSquare, "/scribe"),
    SidebarElement::Navigator(Icon::BookOpen, "/books"),
    SidebarElement::Navigator(Icon::CalenderDays, "/calender"),
    SidebarElement::Seperator,
    SidebarElement::Navigator(Icon::Cog8Tooth, "/settings"),
];

const BACK_ELEMENT: SidebarElement = SidebarElement::Navigator(Icon::DocumentPlus, "/"); // TODO: make "/" to just one segment back

pub(crate) fn init_sidebar_data(cx: &Scope) {
    use_context_provider(&cx, || UseSidebar(SidebarData::Applications));
}

pub fn Sidebar<'a>(cx: Scope) -> Element {
    let sidebar_data = match use_context::<UseSidebar>(&cx) {
        Some(data) => data,
        None => panic!("Couldn't find UseSidebar"),
    };

    let elements = use_state::<Vec<SidebarElement>>(&cx, || vec![]);

    use_effect(&cx, &sidebar_data.read().0, |data| {
        info!("KEK");
        let value = match data {
            SidebarData::Applications => Vec::from(APPLICATION_SIDEBAR),
            SidebarData::SubPage(custom) => {
                let mut vec = Vec::with_capacity(custom.len() + 1);
                vec.push(BACK_ELEMENT);
                vec.extend_from_slice(custom);
                vec
            }
        };
        elements.set(value);
        async {}
    });

    cx.render(rsx! {
        ul {
            class: "flex flex-col shadow h-screen min-h-screen w-16 items-center sticky left-0 top-0",
            rsx! {
                elements.iter().enumerate().map(|(i, elem)| match elem {
                    SidebarElement::Seperator => rsx!(div { key: "{i}", class: "grow" }),
                    SidebarElement::Navigator(icon, path) => rsx!(NavigationSidebarElement {
                        key: "{i}",
                        icon: icon.clone(),
                        path: path,
                    }),
                })
            },
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
