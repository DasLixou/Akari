#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::info;

use crate::{components::sidebar::Sidebar, screens::home::HomeScreen};

mod components;
mod icons;
mod screens;

pub fn launch() {
    info!("Launching Akari UI");
    dioxus::desktop::launch_with_props(
        app,
        AppProps {
            screen: Screen::Home,
        },
        |c| c.with_custom_head("<script src=\"https://cdn.tailwindcss.com\"></script>".to_string()),
    );
}

#[derive(PartialEq)]
pub(crate) enum Screen {
    Home,
}

#[derive(Props, PartialEq)]
pub(crate) struct AppProps {
    screen: Screen,
}

fn app(cx: Scope<AppProps>) -> Element {
    let screen = match cx.props.screen {
        Screen::Home => rsx!(HomeScreen {}),
    };
    cx.render(rsx! {
        main {
            class: "flex",
            Sidebar {},
            screen
        }
    })
}
