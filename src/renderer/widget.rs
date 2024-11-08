use super::painter::Painter;

pub trait Widget<'a, P: Painter<'a>> {
    fn render_widget(&self, painter: &mut P);
}
