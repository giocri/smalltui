use super::painter::Painter;

pub trait Widget<'c, P: Painter<'c>> {
    fn render_widget<'a>(&self, painter: &'a mut P);
}
