use vizia::prelude::*;

const VERSION: &str = "InDev";
const SIDEBAR_ELEMENT_SIZE: Units = Pixels(60.);

fn main() {
    Application::new(|cx| {
        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Akari")
                    .background_color(Color::black())
                    .color(Color::white())
                    .size(SIDEBAR_ELEMENT_SIZE);
                Label::new(cx, "Scribe").size(SIDEBAR_ELEMENT_SIZE);
                Label::new(cx, "Books").size(SIDEBAR_ELEMENT_SIZE);
                Label::new(cx, "Calendar").size(SIDEBAR_ELEMENT_SIZE);
                Label::new(cx, "Settings").size(SIDEBAR_ELEMENT_SIZE);
            })
            .width(SIDEBAR_ELEMENT_SIZE)
            .background_color(Color::rgb(250, 250, 250))
            .border_width(Pixels(1.))
            .border_color(Color::black())
            .overflow(Overflow::Hidden)
            .class("sidebar");
            Label::new(cx, "Content goes here!");
        });
    })
    .title(&format!("Akari | {VERSION}"))
    .inner_size((1404 / 2, 1872 / 2))
    .run();
}
