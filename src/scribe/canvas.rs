use glam::Vec2;
use vizia::{
    context::RelativeResult,
    prelude::*,
    state::{Lens, LensExt},
    vg::Path,
    view::View,
};

use crate::atoms::{atom::Atom, container::AtomContainer};

use super::{brushes::Brush, PageEvent};

pub enum CanvasEvent {
    SelectBrush(Brush),
}

pub static CURRENT_BRUSH: Atom<Brush> = Atom::new();

#[derive(Lens)]
pub struct PageCanvas<L: Lens> {
    delta_mouse: Vec2,
    paths: L,
}

impl<L> PageCanvas<L>
where
    L: Lens<Target = Vec<(Path, Brush)>>,
{
    pub fn new(cx: &mut Context, paths: L) -> Handle<'_, Self> {
        Self {
            paths,
            delta_mouse: Vec2::ZERO,
        }
        .build(cx, |cx| {
            cx.emit(AtomContainer::set(&CURRENT_BRUSH, Brush::Pen))
        })
        .size(Stretch(1.0))
        .id("page_canvas")
    }
}

impl<L> View for PageCanvas<L>
where
    L: Lens<Target = Vec<(Path, Brush)>>,
{
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseDown(_) => {
                if let RelativeResult::Inside((rel_x, rel_y)) =
                    cx.relative_position(cx.mouse.cursorx, cx.mouse.cursory)
                {
                    cx.emit(PageEvent::BeginPath((
                        AtomContainer::lens(&CURRENT_BRUSH).get(cx),
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
                let (rel_x, rel_y) = cx.relative_position(*x, *y).unwrap();
                if meta.target == cx.current() && cx.mouse.left.state == MouseButtonState::Pressed {
                    let mouse = Vec2::new(rel_x, rel_y);
                    if mouse.distance(self.delta_mouse)
                        > AtomContainer::lens(&CURRENT_BRUSH).get(cx).spacing()
                    {
                        self.delta_mouse = mouse;
                        cx.emit(PageEvent::ExtendPath((rel_x, rel_y)));
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

        if let Some(event) = event.take() {
            match event {
                CanvasEvent::SelectBrush(brush) => {
                    cx.emit(AtomContainer::set(&CURRENT_BRUSH, brush));
                }
            }
        }
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let b = cx.bounds();
        canvas.save();
        canvas.translate(b.x, b.y);
        canvas.scissor(0., 0., b.w, b.h);
        for (path, brush) in &mut self.paths.get(cx) {
            canvas.stroke_path(path, &brush.to_paint());
        }
        canvas.restore();
    }
}
