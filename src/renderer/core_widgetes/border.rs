use crate::renderer::{painter::Painter, rect::Rect, widget::Widget, Simble};

pub struct Border {
    top_left: Simble,
    top_right: Simble,
    vertical_left: Simble,
    vertical_right: Simble,
    top_line: Simble,
    bottom_line: Simble,
    bottom_left: Simble,
    bottom_right: Simble,
    width: u16,
    height: u16,
}

impl Border {
    pub fn new(
        top_left: Simble,
        top_right: Simble,
        vertical_left: Simble,
        vertical_right: Simble,
        top_line: Simble,
        bottom_line: Simble,
        bottom_left: Simble,
        bottom_right: Simble,
        width: u16,
        height: u16,
    ) -> Self {
        Self {
            top_left,
            top_right,
            vertical_left,
            vertical_right,
            top_line,
            bottom_line,
            bottom_left,
            bottom_right,
            width,
            height,
        }
    }
}
impl<'a, P: Painter<'a>> Widget<'a, P> for Border {
    fn render_widget<'b>(&self, painter: &'b mut P) {
        let mut line = vec![self.top_line.clone(); self.width as usize];
        line[0] = self.top_left.clone();
        line[self.width as usize - 1] = self.top_right.clone();
        painter.write_simbles(&line, Rect::new(0, 0, self.width, 1));
        line = vec![self.bottom_line.clone(); self.width as usize];
        line[0] = self.bottom_left.clone();
        line[self.width as usize - 1] = self.bottom_right.clone();
        painter.write_simbles(&line, Rect::new(0, self.height - 1, self.width, 1));
        line = vec![self.vertical_left.clone(); self.height as usize - 2];
        painter.write_simbles(&line, Rect::new(0, 1, 1, self.height - 2));
        line = vec![self.vertical_right.clone(); self.height as usize - 2];
        painter.write_simbles(&line, Rect::new(self.width - 1, 1, 1, self.height - 2));
    }
}
