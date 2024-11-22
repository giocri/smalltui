use super::painter::Painter;

pub trait Widget<P: Painter> {
    fn render_widget(&self, painter: &mut P);
}
