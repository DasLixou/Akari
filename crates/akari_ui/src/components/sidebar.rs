use dioxus::{events::MouseEvent, fermi::use_atom_state, prelude::*};

use crate::{
    icons::{Icon, OutlineIcon, SolidIcon},
    Screen, CURRENT_SCREEN,
};

pub fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx! {
        ul {
            class: "flex flex-col shadow h-screen min-h-screen w-16 items-center sticky left-0 top-0",
            SidebarElement {
                class: "",
                onclick: move |_evt| {},
                "Akari"
            },
            NavigationSidebarElement {
                icon: Icon::PencilSquare,
                screen: Screen::Scribe,
            }
            NavigationSidebarElement {
                icon: Icon::BookOpen,
                screen: Screen::Books,
            }
            NavigationSidebarElement {
                icon: Icon::CalenderDays,
                screen: Screen::Calender,
            }
            div {
                class: "grow"
            }
            NavigationSidebarElement {
                icon: Icon::Cog8Tooth,
                screen: Screen::Settings,
            }
        }
    })
}

#[inline_props]
pub fn SidebarElement<'a>(
    cx: Scope,
    class: Option<&'a str>,
    onclick: Option<EventHandler<'a, MouseEvent>>,
    children: Element<'a>,
) -> Element {
    let class = class.unwrap_or_default();
    cx.render(rsx! {
        li {
            button {
                onclick: move |evt| { onclick.as_ref().map(|f| f.call(evt)); },
                class: "flex w-16 h-16 p-3 items-center justify-center overflow-hidden {class}",
                children
            }
        }
    })
}

#[inline_props]
pub fn NavigationSidebarElement(cx: Scope, icon: Icon, screen: Screen) -> Element {
    let current_screen = use_atom_state(&cx, CURRENT_SCREEN);
    let active = current_screen.get() == screen;

    let class = match active {
        true => "bg-black text-white",
        false => "bg-white text-black",
    };
    let icon = icon.clone();
    let icon = match active {
        true => rsx! {
            SolidIcon { icon: icon }
        },
        false => rsx! {
            OutlineIcon { icon: icon }
        },
    };

    cx.render(rsx! {
        SidebarElement {
            class: "{class}",
            onclick: move |_evt| {
                current_screen.set(cx.props.screen.clone())
            },
            icon
        }
    })
}
