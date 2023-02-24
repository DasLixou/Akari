use glam::Vec2;
use vizia::{
    prelude::*,
    state::{Lens, LensExt},
    vg::{Color, LineCap, Paint, Path},
    view::{Canvas, View},
};

use super::PageEvent;

pub struct PageDisplay<L: Lens> {
    delta_mouse: Vec2,
    paths: L,
}

impl<L> PageDisplay<L>
where
    L: Lens<Target = Vec<Path>>,
{
    pub fn new(cx: &mut Context, paths: L) -> Handle<'_, Self> {
        Self {
            paths,
            delta_mouse: Vec2::ZERO,
        }
        .build(cx, |_cx| {})
    }
}

impl<L> View for PageDisplay<L>
where
    L: Lens<Target = Vec<Path>>,
{
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseDown { .. } => {
                cx.emit(PageEvent::BeginPath((cx.mouse.cursorx, cx.mouse.cursory)));
                self.delta_mouse = Vec2::new(cx.mouse.cursorx, cx.mouse.cursory);
                cx.needs_redraw();
                cx.capture();
            }

            WindowEvent::MouseMove(x, y) => {
                if meta.target == cx.current() && cx.mouse.left.state == MouseButtonState::Pressed {
                    let mouse = Vec2::new(cx.mouse.cursorx, cx.mouse.cursory);
                    if mouse.distance(self.delta_mouse) > 12. {
                        self.delta_mouse = mouse;
                        cx.emit(PageEvent::ExtendPath((*x, *y)));
                        cx.needs_redraw();
                    }
                }
            }

            WindowEvent::MouseUp(_) => {
                if meta.target == cx.current() {
                    cx.release();
                }
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        for path in &mut self.paths.get(cx) {
            canvas.stroke_path(
                path,
                &Paint::color(Color::rgba(65, 212, 215, 100))
                    .with_stencil_strokes(false)
                    .with_line_width(12.)
                    .with_line_cap(LineCap::Round)
                    .with_line_join(vizia::vg::LineJoin::Bevel),
            )
        }
    }
}
