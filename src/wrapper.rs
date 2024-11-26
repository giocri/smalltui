use smalltui::renderer::{painter::Painter, rect::Rect, widget::Widget};

pub struct Wrappper<T> {
    inner: T,
}
impl<T> Wrappper<T> {
    pub fn new(a: T) -> Self {
        Self { inner: a }
    }
}
impl<P: Painter, T: Widget<P>> Widget<P> for Wrappper<T> {
    fn render_widget(&self, painter: &mut P) {
        painter.render_widget(&self.inner, Rect::new(0, 0, 255, 255), 0, 0);
    }
}
