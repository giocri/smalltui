use super::painter::Painter;

pub trait Widget<P: Painter>: Sync {
    fn render_widget(&self, painter: &mut P);
}

/*impl<P: Painter, T: Widget<P>> Widget<P> for Box<T> {
    fn render_widget(&self, painter: &mut P) {
        self.as_ref().render_widget(painter);
    }
}*/
