pub mod scribe;
pub mod sidebar_carousel;

use scribe::scribe;
use sidebar_carousel::{carousel::Carousel, item::SidebarItem, sidebar::Sidebar, SidebarCarousel};
use vizia::prelude::*;

const VERSION: &str = "InDev";

fn main() {
    Application::new(|cx| {
        HStack::new(cx, |cx| {
            SidebarCarousel::new(items![
                SidebarItem {
                    text: "Scribe".into(),
                    content: scribe,
                },
                SidebarItem {
                    text: "Books".into(),
                    content: |cx| {
                        Label::new(cx, "All your books");
                    },
                },
                SidebarItem {
                    text: "Calendar".into(),
                    content: |cx| {
                        Label::new(cx, "Calendar");
                    },
                },
                SidebarItem {
                    text: "Settings".into(),
                    content: |cx| {
                        Label::new(cx, "Settings");
                    },
                },
            ])
            .build(cx);

            Sidebar::new(cx);
            Carousel::new(cx);
        });
    })
    .ignore_default_theme()
    .title(&format!("Akari | {VERSION}"))
    .inner_size((1404 / 2, 1872 / 2))
    .run();
}
