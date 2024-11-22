use super::{
    buffer::VecBuffer,
    buffer_mediator::BufferMediator,
    painter::{simple_painter::SimplePainter, Painter},
    BackgroundColor, ForegroundColor, Simble,
};

pub trait Widget<P: Painter> {
    fn render_widget<'a>(&self, painter: &'a mut P);
}
pub trait BasicWidget {
    fn render_widget<'a>(
        &self,
        painter: &'a mut SimplePainter<
            '_,
            VecBuffer<BackgroundColor>,
            VecBuffer<ForegroundColor>,
            VecBuffer<Simble>,
        >,
    );
}
