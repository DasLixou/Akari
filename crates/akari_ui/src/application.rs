use dioxus::prelude::{Element, Scope};

pub trait Application {
    fn routes(cx: Scope) -> Element
    where
        Self: Sized;
}
