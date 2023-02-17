use vizia::{
    prelude::*,
    state::{Lens, LensExt},
    vg::{Color, Paint, Path},
    view::{Canvas, View},
};

use super::PageEvent;

pub struct PageDisplay<L: Lens> {
    paths: L,
}

impl<L> PageDisplay<L>
where
    L: Lens<Target = Vec<Path>>,
{
    pub fn new(cx: &mut Context, paths: L) -> Handle<'_, Self> {
        Self { paths }.build(cx, |_cx| {})
    }
}

impl<L> View for PageDisplay<L>
where
    L: Lens<Target = Vec<Path>>,
{
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::PressDown { .. } => {
                cx.emit(PageEvent::BeginPath((cx.mouse.cursorx, cx.mouse.cursory)));
                cx.needs_redraw();
                cx.capture();
                cx.focus();
            }

            WindowEvent::MouseMove(x, y) => {
                if meta.target == cx.current() && cx.mouse.left.state == MouseButtonState::Pressed {
                    cx.emit(PageEvent::ExtendPath((*x, *y)));
                    cx.needs_redraw();
                }
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        for path in &mut self.paths.get(cx) {
            canvas.stroke_path(path, &Paint::color(Color::black()).with_line_width(1.2))
        }
    }
}