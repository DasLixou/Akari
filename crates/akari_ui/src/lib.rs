#![allow(non_snake_case)]

use dioxus::{fermi::use_atom_state, prelude::*};
use log::info;

use crate::{
    components::sidebar::Sidebar,
    screens::{
        books::BooksScreen, calender::CalenderScreen, scribe::ScribeScreen,
        settings::SettingsScreen,
    },
};

mod components;
mod icons;
mod screens;

static CURRENT_SCREEN: Atom<Screen> = |_| Screen::Scribe;

pub fn launch() {
    info!("Launching Akari UI");
    dioxus::desktop::launch_cfg(app, |c| {
        c.with_custom_head("<script src=\"https://cdn.tailwindcss.com\"></script>".to_string())
    });
}

#[derive(PartialEq, Clone)]
pub enum Screen {
    Scribe,
    Books,
    Calender,
    Settings,
}

fn app(cx: Scope) -> Element {
    let screen = match use_atom_state(&cx, CURRENT_SCREEN).get() {
        Screen::Scribe => rsx!(ScribeScreen {}),
        Screen::Books => rsx!(BooksScreen {}),
        Screen::Calender => rsx!(CalenderScreen {}),
        Screen::Settings => rsx!(SettingsScreen {}),
    };
    cx.render(rsx! {
        main {
            class: "flex select-none",
            Sidebar {},
            screen
        }
    })
}
