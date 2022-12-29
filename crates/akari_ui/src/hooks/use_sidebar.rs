use core::panic;
use std::{cell::RefCell, rc::Rc};

use dioxus::prelude::*;

use crate::sidebar::SidebarData;

/// This hook provides access to the sidebar
pub fn use_sidebar(cx: &ScopeState, data: fn() -> SidebarData<'static>) {
    let sidebar_data = data();
    cx.use_hook(move |_| match cx.consume_context::<Rc<UseSidebar>>() {
        Some(use_sidebar) => {
            use_sidebar.0.replace(sidebar_data);
        }
        None => panic!("Couldn't find UseSidebar"),
    });
}

pub(crate) struct UseSidebar<'a>(pub(crate) RefCell<SidebarData<'a>>);
