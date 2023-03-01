pub mod scribe;
pub mod sidebar_carousel;

use scribe::SCRIBE;
use sidebar_carousel::{
    item::{ItemBehaviour, SidebarItem},
    sidebar::Sidebar,
    BuildClosure, SidebarCarousel,
};
use vizia::prelude::*;

const VERSION: &str = "InDev";

pub enum AppEvent {
    ChangeContent(BuildClosure),
}

#[derive(Lens)]
pub struct AppData {
    main_content: BuildClosure,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        if let Some(event) = event.take() {
            match event {
                AppEvent::ChangeContent(closure) => {
                    self.main_content = closure;
                }
            }
        }
    }
}

fn main() {
    Application::new(|cx| {
        HStack::new(cx, |cx| {
            SidebarCarousel::new(items![
                SidebarItem::Button("Scribe".into(), ItemBehaviour::Page(SCRIBE)),
                SidebarItem::Button("Books".into(), ItemBehaviour::Nothing),
                SidebarItem::Button("Calendar".into(), ItemBehaviour::Nothing),
                SidebarItem::Spacer,
                SidebarItem::Button("Settings".into(), ItemBehaviour::Nothing),
            ])
            .build(cx);

            AppData {
                main_content: BuildClosure(|_| {}),
            }
            .build(cx);

            Sidebar::new(cx);
            Binding::new(cx, AppData::main_content, |cx, lens| {
                let content = lens.get(cx);
                content.build(cx);
            })
        });
    })
    .ignore_default_theme()
    .title(&format!("Akari | {VERSION}"))
    .inner_size((1404 / 2, 1872 / 2))
    .run();
}
