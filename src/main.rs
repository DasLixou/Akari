pub mod sidebar_carousel;

use sidebar_carousel::{carousel::Carousel, sidebar::Sidebar, SidebarCarousel, SidebarItem};
use vizia::prelude::*;

const VERSION: &str = "InDev";

fn main() {
    Application::new(|cx| {
        HStack::new(cx, |cx| {
            SidebarCarousel::new(vec![
                SidebarItem {
                    text: "Akari".into(),
                    content: |cx| {
                        Label::new(cx, "Homepage");
                    },
                },
                SidebarItem {
                    text: "Scribe".into(),
                    content: |cx| {
                        Label::new(cx, "Select Notebook");
                    },
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
    .title(&format!("Akari | {VERSION}"))
    .inner_size((1404 / 2, 1872 / 2))
    .run();
}
