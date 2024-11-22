use super::painter::Painter;

pub trait Widget<P: Painter>: Sync + Send {
    fn render_widget(&self, painter: &mut P);
}
