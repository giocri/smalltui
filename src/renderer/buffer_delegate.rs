use crate::renderer::buffer::Buffer;

use super::rect::Rect;
pub struct BufferMediator {
    area: Rect,
    offset_x: u16,
    offset_y: u16,
}
impl BufferMediator {
    pub fn new(area: Rect, offset_x: u16, offset_y: u16) -> BufferMediator {
        BufferMediator {
            area: area,
            offset_x: offset_x,
            offset_y: offset_y,
        }
    }
    pub fn draw<T: Default + Sized + Copy, B: Buffer<T>>(
        &mut self,
        data: &[T],
        area: Rect,
        buffer: &mut B,
    ) {
        let in_scope = self.get_inscope(&area);
        let removed_x = (in_scope.x - area.x) as usize;
        let removed_y = (in_scope.y - area.y) as usize;
        for i in 0..in_scope.height as usize {
            let starting_index = (i + removed_y) * (area.width as usize) + removed_x;
            buffer.draw_line(
                &data[starting_index..(starting_index + in_scope.width as usize)],
                in_scope.x - self.offset_x,
                in_scope.y - self.offset_y,
                in_scope.width,
            );
        }
    }
    pub fn get_inscope(&self, area: &Rect) -> Rect {
        let window = Rect::new(
            self.offset_x,
            self.offset_y,
            self.area.width,
            self.area.height,
        );
        window.crop(area)
    }
    pub fn generate_inner(&self, area: &Rect, offset_x: u16, offset_y: u16) -> Self {
        BufferMediator {
            area: self.get_inscope(&area),
            offset_x: self.offset_x + offset_x,
            offset_y: self.offset_x + offset_y,
        }
    }
}
