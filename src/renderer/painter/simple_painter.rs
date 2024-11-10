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
impl<'b, 'a: 'b, A: Buffer<BackgroundColor>, B: Buffer<ForegroundColor>, C: Buffer<Simble>>
    SimplePainter<'b, A, B, C>
{
    pub fn new(
        background: &'a mut A,
        foreground: &'a mut B,
        text: &'a mut C,
        mediator: BufferMediator,
    ) -> SimplePainter<'b, A, B, C> {
        SimplePainter {
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
impl<'a, A: Buffer<BackgroundColor>, B: Buffer<ForegroundColor>, C: Buffer<Simble>> Painter
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
    fn get_painter(&mut self, mediator: BufferMediator) -> impl Painter {
        SimplePainter::new(
            &mut *self.background,
            &mut *self.foreground,
            &mut *self.text,
            mediator,
        )
    }
    fn area(&self) -> Rect {
        return self.mediator.size();
    }
}
