use vizia::{prelude::*, vg::Path};

use crate::{
    closures::{BuildClosure, EventClosure},
    items,
    sidebar_carousel::{
        item::{ItemBehaviour, SidebarItem},
        SidebarCarouselEvent,
    },
};

use self::{
    brushes::Brush,
    canvas::{CanvasEvent, PageCanvas},
};

pub mod brushes;
pub mod canvas;

pub enum PageEvent {
    BeginPath((Brush, f32, f32)),
    ExtendPath((f32, f32)),
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
        })
    }
}

pub const SCRIBE: BuildClosure = BuildClosure(scribe);

pub fn scribe(cx: &mut Context) {
    cx.emit(SidebarCarouselEvent::ShowSubItems(items![
        SidebarItem::Button(
            "Pen".into(),
            ItemBehaviour::Action(EventClosure(|cx| {
                if let Some(canvas) = cx.resolve_entity_identifier("page_canvas") {
                    cx.emit_custom(
                        Event::new(CanvasEvent::SelectBrush(Brush::Pen))
                            .target(canvas)
                            .origin(cx.current())
                            .propagate(Propagation::Subtree),
                    );
                }
            }))
        ),
        SidebarItem::Button(
            "Marker".into(),
            ItemBehaviour::Action(EventClosure(|cx| {
                if let Some(canvas) = cx.resolve_entity_identifier("page_canvas") {
                    cx.emit_custom(
                        Event::new(CanvasEvent::SelectBrush(Brush::Marker))
                            .target(canvas)
                            .origin(cx.current())
                            .propagate(Propagation::Subtree),
                    );
                }
            })),
        )
    ]));
    Page::default().build(cx);
    PageCanvas::new(cx, Page::paths);
}
