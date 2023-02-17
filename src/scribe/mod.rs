use vizia::{prelude::*, vg::Path};

use self::display::PageDisplay;

pub mod display;

pub enum PageEvent {
    BeginPath((f32, f32)),
    ExtendPath((f32, f32)),
}

#[derive(Default, Lens)]
pub struct Page {
    paths: Vec<Path>,
}

impl Model for Page {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|page_event, _| match page_event {
            PageEvent::BeginPath((x, y)) => {
                println!("begin");
                let mut path = Path::new();
                path.move_to(*x, *y);
                self.paths.push(path);
            }
            PageEvent::ExtendPath((x, y)) => {
                println!("extend");
                self.paths.last_mut().unwrap().line_to(*x, *y);
            }
        })
    }
}

pub fn scribe(cx: &mut Context) {
    Page::default().build(cx);
    PageDisplay::new(cx, Page::paths);
}
