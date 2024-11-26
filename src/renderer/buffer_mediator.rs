use crate::renderer::buffer::Buffer;

use super::rect::Rect;
#[derive(Clone, Copy)]
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
    pub fn write<T: Default + Sized + Clone + Send + Sync, B: Buffer<T>>(
        &self,
        data: &[T],
        area: Rect,
        buffer: &mut B,
    ) {
        //println!("requested area:{:?}", area);
        let visible = self.get_visible_region(&area);
        //println!("visible area{:?}", visible);
        if visible.width == 0 || visible.height == 0 {
            return;
        }
        let first_x = (visible.x - area.x) as usize; //numbers of columns of the requested area that are hidden
        let first_y = (visible.y - area.y) as usize; //numbers of rows of the requested area that are hidden
        let screen_area = self.map_to_screen_space(&visible);
        //println!("screen visible area{:?}", screen_area);
        for i in 0..visible.height as usize {
            let y_to_draw = i + first_y; //current row of the request being writtent
            let starting_index = y_to_draw * (area.width as usize) + first_x; //index of the first element of the row to write
                                                                              //println!("drawind with x up to{:?}", screen_area.x + visible.width);
                                                                              //println!("on y:{:?}", screen_area.y + (i as u16));
            buffer.draw_line(
                &data[starting_index..(starting_index + visible.width as usize)], //slice of data to draw
                screen_area.x,
                screen_area.y + (i as u16),
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
    fn map_to_screen_space(&self, area: &Rect) -> Rect {
        Rect::new(
            area.x - self.offset_x + self.area.x,
            area.y - self.offset_y + self.area.y,
            area.width,
            area.height,
        )
    }
    pub fn generate_inner(&self, area: &Rect, offset_x: u16, offset_y: u16) -> Self {
        BufferMediator {
            area: self.map_to_screen_space(&self.get_visible_region(&area)),
            offset_x: self.offset_x + offset_x,
            offset_y: self.offset_y + offset_y,
        }
    }
    pub fn size(&self) -> Rect {
        return Rect::new(0, 0, self.area.width, self.area.height);
    }
}
