use vizia::vg::{Color, LineCap, Paint};

#[derive(Clone, PartialEq)]
pub enum Brush {
    Pen,
    Marker,
}

impl Brush {
    #[inline]
    pub const fn to_paint(&self) -> Paint {
        match self {
            Brush::Pen => PEN,
            Brush::Marker => MARKER,
        }
    }

    #[inline]
    pub const fn spacing(&self) -> f32 {
        match self {
            Brush::Pen => 1.2,
            Brush::Marker => 12.,
        }
    }
}

pub const PEN: Paint = Paint::color(Color::black()).with_line_width(1.2);

pub const MARKER: Paint = Paint::color(Color::rgbaf(
    65. / 255.,
    212. / 255.,
    215. / 255.,
    100. / 255.,
))
.with_stencil_strokes(false)
.with_line_width(12.)
.with_line_cap(LineCap::Square)
.with_line_join(vizia::vg::LineJoin::Bevel);
