use crate::renderer::{
    buffer::Buffer, buffer_mediator::BufferMediator, rect::Rect, BackgroundColor, ForegroundColor,
    Simble,
};

use super::Painter;

pub struct SimplePainter<
    'a,
    A: Buffer<BackgroundColor>,
    B: Buffer<ForegroundColor>,
    C: Buffer<Simble>,
> {
    background: &'a mut A,
    foreground: &'a mut B,
    text: &'a mut C,
    mediator: BufferMediator,
}
impl<'a, A: Buffer<BackgroundColor>, B: Buffer<ForegroundColor>, C: Buffer<Simble>>
    SimplePainter<'a, A, B, C>
{
    pub fn new(
        background: &'a mut A,
        foreground: &'a mut B,
        text: &'a mut C,
        mediator: BufferMediator,
    ) -> Self {
        Self {
            background,
            foreground,
            text,
            mediator,
        }
    }
    fn fill<T: Default + Sized + Clone>(
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
impl<'a, A: Buffer<BackgroundColor>, B: Buffer<ForegroundColor>, C: Buffer<Simble>> Painter<'a>
    for SimplePainter<'a, A, B, C>
{
    fn background_fill(&mut self, color: BackgroundColor, area: Option<Rect>) {
        Self::fill::<BackgroundColor>(&self.mediator, color, area, self.background);
    }
    fn foreground_fill(&mut self, color: ForegroundColor, area: Option<Rect>) {
        Self::fill::<ForegroundColor>(&self.mediator, color, area, self.foreground);
    }
    fn simble_fill(&mut self, color: Simble, area: Option<Rect>) {
        Self::fill::<Simble>(&self.mediator, color, area, self.text);
    }
    fn write_simbles(&mut self, text: &[Simble], area: Rect) {
        self.mediator.write(text, area, self.text);
    }
    fn write_background_color(&mut self, color: &[BackgroundColor], area: Rect) {
        self.mediator.write(color, area, self.background);
    }
    fn write_foreground_color(&mut self, color: &[ForegroundColor], area: Rect) {
        self.mediator.write(color, area, self.foreground);
    }

    fn delegate_painter(
        &'a mut self,
        area: &Rect,
        offset_x: u16,
        offset_y: u16,
    ) -> SimplePainter<'a, A, B, C> {
        SimplePainter::new(
            self.background,
            self.foreground,
            self.text,
            self.mediator.generate_inner(area, offset_x, offset_y),
        )
    }
}
