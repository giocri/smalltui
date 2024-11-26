use smalltui::renderer::{
    painter::{Painter, TextPainer},
    rect::Rect,
    widget::Widget,
};

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
        painter.render_widget(&self.inner, Rect::new(2, 2, 8, 5), 0, 0);
    }
}
#[derive(Clone)]
pub struct Count {}
impl<P: Painter + TextPainer> Widget<P> for Count {
    fn render_widget(&self, painter: &mut P) {
        for i in 0..20 {
            painter.write_paragraph(i.to_string().as_str(), 0, i, None);
        }
    }
}
