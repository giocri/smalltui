use crate::renderer::{
    painter::Painter, rect::Rect, widget::Widget, BackgroundColor, Direction, ForegroundColor,
    Simble,
};

pub struct ListWidget<T> {
    elements: Vec<T>,
    element_width: u16,
    element_height: u16,
    scroll: u16,
}

impl<T> ListWidget<T> {
    pub fn new(elements: Vec<T>, scroll: u16, element_width: u16, element_height: u16) -> Self {
        Self {
            elements,
            element_width,
            element_height,
            scroll,
        }
    }
}
impl<P: Painter, T: Widget<P>> Widget<P> for ListWidget<T> {
    fn render_widget(&self, painter: &mut P) {
        let mut y = 0;
        for e in self.elements.iter() {
            let area = Rect::new(0, y, self.element_width, self.element_height);
            painter.render_widget(e, area, 0, self.scroll);
            y += self.element_height + 1;
        }
    }
}
