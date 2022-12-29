use dioxus::{fermi::use_atom_state, prelude::*};

use crate::{
    icons::{Icon, OutlineIcon, SolidIcon},
    Screen, CURRENT_SCREEN,
};

pub fn Sidebar(cx: Scope) -> Element {
    cx.render(rsx! {
        ul {
            class: "flex flex-col shadow h-screen min-h-screen w-16 items-center sticky left-0 top-0",
            li {
                class: "flex w-16 h-16 p-3 items-center justify-center overflow-hidden",
                "Akari"
            },
            SidebarElement {
                icon: Icon::PencilSquare,
                screen: Screen::Scribe,
            }
            SidebarElement {
                icon: Icon::BookOpen,
                screen: Screen::Books,
            }
            SidebarElement {
                icon: Icon::CalenderDays,
                screen: Screen::Calender,
            }
            div {
                class: "grow"
            }
            SidebarElement {
                icon: Icon::Cog8Tooth,
                screen: Screen::Settings,
            }
        }
    })
}

#[derive(Props, PartialEq)]
pub struct SidebarElementProps {
    icon: Icon,
    screen: Screen,
}

pub fn SidebarElement(cx: Scope<SidebarElementProps>) -> Element {
    let current_screen = use_atom_state(&cx, CURRENT_SCREEN);
    let active = current_screen.get() == &cx.props.screen;

    let class = match active {
        true => {
            "flex w-16 h-16 p-3 items-center justify-center overflow-hidden bg-black text-white"
        }
        false => {
            "flex w-16 h-16 p-3 items-center justify-center overflow-hidden bg-white text-black"
        }
    };
    let icon = cx.props.icon.clone();
    let icon = match active {
        true => rsx! {
            SolidIcon { icon: icon }
        },
        false => rsx! {
            OutlineIcon { icon: icon }
        },
    };

    cx.render(rsx! {
        li {
            button {
                class: "{class}",
                onclick: move |_evt| {
                    current_screen.set(cx.props.screen.clone())
                },
                icon
            }
        }
    })
}
