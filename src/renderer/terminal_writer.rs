use super::{
    buffer::Buffer, buffer_mediator::BufferMediator, painter::simple_painter::SimplePainter,
    BackgroundColor, ForegroundColor, Simble,
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
    pub fn buffers(&mut self) -> (&mut A, &mut B, &mut C) {
        (&mut self.background, &mut self.foreground, &mut self.text)
    }
    pub fn get_painter<'b, 'c: 'b>(
        &'c mut self,
        mediator: BufferMediator,
    ) -> SimplePainter<'b, A, B, C> {
        SimplePainter::new(
            &mut self.background,
            &mut self.foreground,
            &mut self.text,
            mediator,
        )
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
