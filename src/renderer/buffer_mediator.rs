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
    pub fn write<T: Default + Sized + Clone, B: Buffer<T>>(
        &self,
        data: &[T],
        area: Rect,
        buffer: &mut B,
    ) {
        let visible = self.get_visible_region(&area);
        if visible.width == 0 || visible.height == 0 {
            return;
        }
        let removed_x = (visible.x - area.x) as usize;
        let removed_y = (visible.y - area.y) as usize;

        for i in 0..visible.height as usize {
            let y_to_draw = i + removed_y;
            let starting_index = y_to_draw * (area.width as usize) + removed_x;
            buffer.draw_line(
                &data[starting_index..(starting_index + visible.width as usize)],
                visible.x - self.offset_x + self.area.x,
                visible.y - self.offset_y + self.area.x,
                visible.width,
            );
        }
    }
    pub fn get_visible_region(&self, area: &Rect) -> Rect {
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
            area: self.get_visible_region(&area),
            offset_x: self.offset_x + offset_x,
            offset_y: self.offset_x + offset_y,
        }
    }
}
