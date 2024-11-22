use super::{buffer::VecBuffer, painter::Painter, BackgroundColor, ForegroundColor, Simble};

pub trait Widget<P: Painter> {
    fn render_widget<'a>(&self, painter: &'a mut P);
}
