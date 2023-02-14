pub mod sidebar;

use sidebar::create_sidebar;
use vizia::prelude::*;

const VERSION: &str = "InDev";

fn main() {
    Application::new(|cx| {
        HStack::new(cx, |cx| {
            create_sidebar(cx);
            Label::new(cx, "Content goes here!");
        });
    })
    .title(&format!("Akari | {VERSION}"))
    .inner_size((1404 / 2, 1872 / 2))
    .run();
}
