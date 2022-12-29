#![allow(non_snake_case)]

use components::sidebar::init_sidebar_data;
use dioxus::prelude::*;
use log::info;

use crate::{
    components::sidebar::Sidebar,
    screens::{
        books::BooksScreen, calender::CalenderScreen, home::HomeScreen, scribe::ScribeScreen,
        settings::SettingsScreen,
    },
};

mod components;
mod hooks;
mod icons;
mod screen;
mod screens;
mod sidebar;

static STYLES: &'static str = include_str!("../tailwind/generated.css");

pub fn launch() {
    info!("Launching Akari UI");
    dioxus::desktop::launch_cfg(app, |c| c);
}

fn app(cx: Scope) -> Element {
    init_sidebar_data(&cx);

    cx.render(rsx! {
        style {
            vec![STYLES]
        },

        main {
            class: "flex",
            Router {
                Sidebar {},

                Route { to: "/", HomeScreen {} }
                Route { to: "/scribe", ScribeScreen {} }
                Route { to: "/books", BooksScreen {} }
                Route { to: "/calender", CalenderScreen {} }
                Route { to: "/settings", SettingsScreen {} }
                Route { to: "", HomeScreen {} }
            }
        }
    })
}
