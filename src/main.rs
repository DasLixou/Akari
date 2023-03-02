pub mod atoms;
pub mod closures;
pub mod scribe;
pub mod sidebar_carousel;

use atoms::container::AtomContainer;
use closures::{BuildClosure, InitClosure};
use scribe::SCRIBE;
use sidebar_carousel::{
    item::{ItemBehaviour, SidebarItem},
    sidebar::Sidebar,
    SidebarCarousel,
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
        AtomContainer::new().build(cx);

        HStack::new(cx, |cx| {
            SidebarCarousel::new(items![
                SidebarItem::Button(
                    "Scribe".into(),
                    ItemBehaviour::Page(SCRIBE),
                    InitClosure(|cx| {
                        let index = cx.index;
                        cx.checked(SidebarCarousel::selected.map(move |i| *i == index));
                    }),
                ),
                SidebarItem::Button(
                    "Books".into(),
                    ItemBehaviour::Nothing,
                    InitClosure(|cx| {
                        let index = cx.index;
                        cx.checked(SidebarCarousel::selected.map(move |i| *i == index));
                    }),
                ),
                SidebarItem::Button(
                    "Calendar".into(),
                    ItemBehaviour::Nothing,
                    InitClosure(|cx| {
                        let index = cx.index;
                        cx.checked(SidebarCarousel::selected.map(move |i| *i == index));
                    }),
                ),
                SidebarItem::Spacer,
                SidebarItem::Button(
                    "Settings".into(),
                    ItemBehaviour::Nothing,
                    InitClosure(|cx| {
                        let index = cx.index;
                        cx.checked(SidebarCarousel::selected.map(move |i| *i == index));
                    }),
                ),
            ])
            .build(cx);

            AppData {
                main_content: BuildClosure(|_| {}),
            }
            .build(cx);

            Sidebar::new(cx);
            Binding::new(cx, AppData::main_content, |cx, lens| {
                let content = lens.get(cx);
                content.run(cx);
            })
        });
    })
    .ignore_default_theme()
    .title(&format!("Akari | {VERSION}"))
    .inner_size((1404 / 2, 1872 / 2))
    .run();
}
