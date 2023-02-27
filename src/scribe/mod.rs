use vizia::{prelude::*, vg::Path};

use self::{brushes::Brush, display::PageDisplay};

pub mod brushes;
pub mod display;

pub enum PageEvent {
    BeginPath((Brush, f32, f32)),
    ExtendPath((f32, f32)),
    ContinuePath((f32, f32)),
}

#[derive(Default, Lens)]
pub struct Page {
    paths: Vec<(Path, Brush)>,
}

impl Model for Page {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|page_event, _| match page_event {
            PageEvent::BeginPath((paint, x, y)) => {
                let mut path = Path::new();
                path.move_to(*x, *y);
                self.paths.push((path, paint.clone()));
            }
            PageEvent::ExtendPath((x, y)) => {
                self.paths.last_mut().unwrap().0.line_to(*x, *y);
            }
            PageEvent::ContinuePath((x, y)) => {
                self.paths.last_mut().unwrap().0.move_to(*x, *y);
            }
        })
    }
}

pub fn scribe(cx: &mut Context) {
    Page::default().build(cx);
    PageDisplay::new(cx, Page::paths);
}
