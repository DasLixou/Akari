#![allow(non_snake_case)]

use const_format::formatcp;
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

static STYLES: &'static str = include_str!("../tailwind/generated.css");

static CURRENT_SCREEN: Atom<Screen> = |_| Screen::Scribe;

pub fn launch() {
    info!("Launching Akari UI");
    dioxus::desktop::launch_cfg(app, |c| c);
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
        style {
            vec![STYLES]
        },

        main {
            class: "flex select-none",
            Sidebar {},
            screen
        }
    })
}
