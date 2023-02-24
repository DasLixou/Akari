use glam::Vec2;
use vizia::{
    prelude::*,
    state::{Lens, LensExt},
    vg::Path,
    view::{Canvas, View},
};

use super::{brushes::Brush, PageEvent};

pub struct PageDisplay<L: Lens> {
    delta_mouse: Vec2,
    current_brush: Brush,
    paths: L,
}

impl<L> PageDisplay<L>
where
    L: Lens<Target = Vec<(Path, Brush)>>,
{
    pub fn new(cx: &mut Context, paths: L) -> Handle<'_, Self> {
        Self {
            paths,
            delta_mouse: Vec2::ZERO,
            current_brush: Brush::Pen,
        }
        .build(cx, |_cx| {})
    }
}

impl<L> View for PageDisplay<L>
where
    L: Lens<Target = Vec<(Path, Brush)>>,
{
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::KeyDown(code, _) => {
                match code {
                    Code::KeyM => {
                        self.current_brush = Brush::Marker;
                    }
                    Code::KeyP => self.current_brush = Brush::Pen,
                    _ => {}
                };
            }
            WindowEvent::MouseDown(_) => {
                cx.emit(PageEvent::BeginPath((
                    self.current_brush.clone(),
                    cx.mouse.cursorx,
                    cx.mouse.cursory,
                )));
                self.delta_mouse = Vec2::new(cx.mouse.cursorx, cx.mouse.cursory);
                cx.needs_redraw();
                cx.capture();
                cx.focus(); // TODO: can we make this to work before first click?
            }

            WindowEvent::MouseMove(x, y) => {
                if meta.target == cx.current() && cx.mouse.left.state == MouseButtonState::Pressed {
                    let mouse = Vec2::new(cx.mouse.cursorx, cx.mouse.cursory);
                    if mouse.distance(self.delta_mouse) > self.current_brush.spacing() {
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
        for (path, brush) in &mut self.paths.get(cx) {
            canvas.stroke_path(path, &brush.to_paint());
        }
    }
}
