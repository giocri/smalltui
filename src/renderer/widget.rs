use super::painter::Painter;

pub trait Widget<P: Painter> {
    fn render_widget<'a>(&self, painter: &'a mut P);
}
