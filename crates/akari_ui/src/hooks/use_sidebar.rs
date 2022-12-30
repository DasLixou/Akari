use core::panic;
use std::cell::RefCell;

use dioxus::prelude::*;

use crate::sidebar::SidebarData;

/// This hook provides access to the sidebar
pub fn use_sidebar(cx: &ScopeState, data: fn() -> SidebarData<'static>) {
    let sidebar = match use_context::<UseSidebar>(&cx) {
        Some(use_sidebar) => use_sidebar,
        None => panic!("Couldn't find UseSidebar"),
    };
    cx.use_hook(|_| sidebar.write().0.replace(data()));
}

pub(crate) struct UseSidebar<'a>(pub(crate) RefCell<SidebarData<'a>>);
