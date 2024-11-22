use super::{
    buffer::Buffer, buffer_mediator::BufferMediator, painter::Painter, rect::Rect, BackgroundColor,
    ForegroundColor, Simble,
};
use crossterm::{
    cursor,
    style::{self, Colors, SetColors},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, Write};
pub struct TerminalWriter<A: Buffer<BackgroundColor>, B: Buffer<ForegroundColor>, C: Buffer<Simble>>
{
    background: A,
    previous_background: A,
    foreground: B,
    previous_foreground: B,
    text: C,
    previous_text: C,
    stream: std::io::Stdout,
    mediator_stack: Vec<BufferMediator>,
}
impl<A: Buffer<BackgroundColor>, B: Buffer<ForegroundColor>, C: Buffer<Simble>>
    TerminalWriter<A, B, C>
{
    pub fn new(width: u16, height: u16, stream: std::io::Stdout) -> TerminalWriter<A, B, C> {
        TerminalWriter {
            background: A::new(width, height),
            previous_background: A::new(width, height),
            foreground: B::new(width, height),
            previous_foreground: B::new(width, height),
            text: C::new(width, height),
            previous_text: C::new(width, height),
            stream,
            mediator_stack: Vec::new(),
        }
    }
    pub fn prepare_area(&mut self) {
        self.stream.execute(EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();
        self.clear();
    }
    pub fn clear(&mut self) {
        self.stream.execute(Clear(ClearType::All)).unwrap();
    }
    pub fn flush_frame(&mut self) -> io::Result<()> {
        let area = self.background.area();
        let stdout = &mut self.stream;
        for y in 0..area.height {
            for x in 0..area.width {
                let new = (
                    self.background[(x, y)],
                    self.foreground[(x, y)],
                    self.text[(x, y)].clone(),
                );
                let previous = (
                    self.previous_background[(x, y)],
                    self.previous_foreground[(x, y)],
                    self.previous_text[(x, y)].clone(),
                );
                if new != previous {
                    stdout
                        .queue(cursor::MoveTo(x, y))?
                        .queue(SetColors(Colors::new(new.1 .0, new.0 .0)))?
                        .queue(style::Print(new.2 .0))?;
                }
            }
        }
        stdout.flush()?;
        std::mem::swap(&mut self.background, &mut self.previous_background);
        std::mem::swap(&mut self.foreground, &mut self.previous_foreground);
        std::mem::swap(&mut self.text, &mut self.previous_text);
        self.background.reset();
        self.foreground.reset();
        self.text.reset();
        Ok(())
    }
    pub fn resize(&mut self, width: u16, height: u16) {
        self.background.resize(width, height);
        self.previous_background.resize(width, height);
        self.foreground.resize(width, height);
        self.previous_foreground.resize(width, height);
        self.text.resize(width, height);
        self.previous_text.resize(width, height);
    }
    fn fill<T: Default + Sized + Clone + Send + Sync>(
        mediator: &BufferMediator,
        content: T,
        area: Option<Rect>,
        buff: &mut impl Buffer<T>,
    ) {
        let buffer_area = buff.area();
        let area = area.unwrap_or(buffer_area);
        let slice_size = area.height as usize * area.width as usize;
        let fill = vec![content; slice_size];
        mediator.write(fill.as_slice(), area, buff);
    }
}
impl<A: Buffer<BackgroundColor>, B: Buffer<ForegroundColor>, C: Buffer<Simble>> Drop
    for TerminalWriter<A, B, C>
{
    fn drop(&mut self) {
        self.stream.queue(style::ResetColor).unwrap();
        self.stream.queue(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        self.stream.flush().unwrap();
    }
}
impl<A: Buffer<BackgroundColor>, B: Buffer<ForegroundColor>, C: Buffer<Simble>> Painter
    for TerminalWriter<A, B, C>
{
    fn background_fill(&mut self, color: BackgroundColor, area: Option<super::rect::Rect>) {
        let default_mediator = BufferMediator::new(self.background.area(), 0, 0);

        Self::fill::<BackgroundColor>(
            &self.mediator_stack.last().unwrap_or(&default_mediator),
            color,
            area,
            &mut self.background,
        );
    }

    fn foreground_fill(&mut self, color: ForegroundColor, area: Option<super::rect::Rect>) {
        let default_mediator = BufferMediator::new(self.background.area(), 0, 0);

        Self::fill::<ForegroundColor>(
            &self.mediator_stack.last().unwrap_or(&default_mediator),
            color,
            area,
            &mut self.foreground,
        );
    }

    fn simble_fill(&mut self, color: Simble, area: Option<super::rect::Rect>) {
        let default_mediator = BufferMediator::new(self.background.area(), 0, 0);
        Self::fill::<Simble>(
            &self.mediator_stack.last().unwrap_or(&default_mediator),
            color,
            area,
            &mut self.text,
        );
    }

    fn write_simbles(&mut self, text: &[Simble], area: super::rect::Rect) {
        let default_mediator = BufferMediator::new(self.background.area(), 0, 0);
        let mediator = &self.mediator_stack.last().unwrap_or(&default_mediator);
        mediator.write(text, area, &mut self.text);
    }

    fn write_background_color(&mut self, color: &[BackgroundColor], area: super::rect::Rect) {
        let default_mediator = BufferMediator::new(self.background.area(), 0, 0);
        let mediator = &self.mediator_stack.last().unwrap_or(&default_mediator);
        mediator.write(color, area, &mut self.background);
    }

    fn write_foreground_color(&mut self, color: &[ForegroundColor], area: super::rect::Rect) {
        let default_mediator = BufferMediator::new(self.background.area(), 0, 0);
        let mediator = &self.mediator_stack.last().unwrap_or(&default_mediator);
        mediator.write(color, area, &mut self.foreground);
    }

    fn area(&self) -> super::rect::Rect {
        let default_mediator = BufferMediator::new(self.background.area(), 0, 0);
        let mediator = &self.mediator_stack.last().unwrap_or(&default_mediator);
        mediator.size()
    }

    fn render_widget(
        &mut self,
        widget: &impl super::widget::Widget<TerminalWriter<A, B, C>>,
        area: super::rect::Rect,
        scroll_x: u16,
        scroll_y: u16,
    ) {
        let default_mediator = BufferMediator::new(self.background.area(), 0, 0);
        let mediator = &self.mediator_stack.last().unwrap_or(&default_mediator);
        self.mediator_stack
            .push(mediator.generate_inner(&area, scroll_x, scroll_y));
        widget.render_widget(self);
        self.mediator_stack.pop();
    }
}
