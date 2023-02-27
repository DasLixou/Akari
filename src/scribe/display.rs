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
    outside: bool,
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
            outside: false,
        }
        .build(cx, |_cx| {})
        .size(Stretch(1.0))
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
                if let Some((rel_x, rel_y)) =
                    cx.bounds_relative_position(cx.mouse.cursorx, cx.mouse.cursory)
                {
                    cx.emit(PageEvent::BeginPath((
                        self.current_brush.clone(),
                        rel_x,
                        rel_y,
                    )));
                    self.delta_mouse = Vec2::new(rel_x, rel_y);
                    cx.needs_redraw();
                    cx.capture();
                    cx.focus(); // TODO: can we make this to work before first click?
                }
            }

            WindowEvent::MouseMove(x, y) => {
                if let Some((rel_x, rel_y)) = cx.bounds_relative_position(*x, *y) {
                    if meta.target == cx.current()
                        && cx.mouse.left.state == MouseButtonState::Pressed
                    {
                        let mouse = Vec2::new(rel_x, rel_y);
                        if mouse.distance(self.delta_mouse) > self.current_brush.spacing() {
                            self.delta_mouse = mouse;
                            if self.outside {
                                self.outside = false;
                                cx.emit(PageEvent::ContinuePath((rel_x, rel_y)));
                            } else {
                                cx.emit(PageEvent::ExtendPath((rel_x, rel_y)));
                            }
                            cx.needs_redraw();
                        }
                    }
                } else {
                    if !self.outside {
                        self.outside = true;
                    }
                }
            }

            WindowEvent::MouseUp(_) => {
                if meta.target == cx.current() {
                    self.outside = false;
                    cx.release();
                }
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let b = cx.bounds();
        canvas.save();
        canvas.translate(b.x, b.y);
        for (path, brush) in &mut self.paths.get(cx) {
            canvas.stroke_path(path, &brush.to_paint());
        }
        canvas.restore();
    }
}
