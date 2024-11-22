use compact_str::ToCompactString;

//use super::buffer_mediator::BufferMediator;
use super::{
    buffer_mediator::BufferMediator, rect::Rect, widget::Widget, BackgroundColor, ForegroundColor,
    Simble,
};

pub trait Painter: Sized {
    fn background_fill(&mut self, color: BackgroundColor, area: Option<Rect>);
    fn foreground_fill(&mut self, color: ForegroundColor, area: Option<Rect>);
    fn simble_fill(&mut self, color: Simble, area: Option<Rect>);
    fn write_simbles(&mut self, text: &[Simble], area: Rect);
    fn write_background_color(&mut self, color: &[BackgroundColor], area: Rect);
    fn write_foreground_color(&mut self, color: &[ForegroundColor], area: Rect);
    fn area(&self) -> Rect;
    fn render_widget(
        &mut self,
        widget: &impl Widget<Self>,
        area: Rect,
        scroll_x: u16,
        scroll_y: u16,
    );
}
pub trait TextPainer {
    fn write_text_line(&mut self, text: &str, x: u16, y: u16);
    fn write_paragraph(&mut self, text: &str, x: u16, y: u16, line_break: Option<u16>);
}
impl<T: Painter> TextPainer for T {
    fn write_text_line(&mut self, text: &str, x: u16, y: u16) {
        let simblevector: Vec<Simble> = text
            .chars()
            .map(|c| Simble(c.to_compact_string()))
            .collect();
        self.write_simbles(
            simblevector.as_slice(),
            Rect::new(x, y, simblevector.len() as u16, 1),
        );
    }
    fn write_paragraph(&mut self, text: &str, x: u16, y: u16, line_break: Option<u16>) {
        let mut curret_y = y;
        match line_break {
            Some(breaking_point) => {
                let mut simbles = Vec::new();
                let mut line_lenght = 0;
                for c in text.chars() {
                    if c == 0xA as char || line_lenght > breaking_point {
                        self.write_simbles(
                            simbles.as_slice(),
                            Rect::new(x, curret_y, simbles.len() as u16, 1),
                        );
                        simbles = Vec::new();
                        curret_y += 1;
                        line_lenght = 0;
                    }
                    if c != 0xA as char {
                        simbles.push(Simble(c.to_compact_string()));
                        line_lenght += 1;
                    }
                }
                if !simbles.is_empty() {
                    self.write_simbles(
                        simbles.as_slice(),
                        Rect::new(x, curret_y, simbles.len() as u16, 1),
                    );
                }
            }
            None => {
                for line in text.lines() {
                    self.write_text_line(line, x, curret_y);
                }
            }
        }
    }
}
