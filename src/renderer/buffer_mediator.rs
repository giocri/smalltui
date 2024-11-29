use crate::renderer::buffer::Buffer;

use super::rect::Rect;
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
        let visible = self.get_visible_region(&area);
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
        let client_area = self.map_to_screen_space(&self.get_visible_region(&area));
        let offset_x = if client_area.x == self.area.x {
            offset_x + area.width - client_area.width
        } else {
            offset_x
        };
        let offset_y = if client_area.y == self.area.y {
            offset_y + area.height - client_area.height
        } else {
            offset_y
        };
        BufferMediator {
            area: client_area,
            offset_x: offset_x,
            offset_y: offset_y,
        }
    }
    pub fn size(&self) -> Rect {
        return Rect::new(0, 0, self.area.width, self.area.height);
    }
}
#[cfg(test)]
mod tests {

    use crate::renderer::buffer::VecBuffer;

    use super::*;
    #[test]
    fn test_new() {
        let area = Rect::new(0, 0, 100, 100);
        let mediator = BufferMediator::new(area, 10, 20);
        assert_eq!(mediator.area, area);
        assert_eq!(mediator.offset_x, 10);
        assert_eq!(mediator.offset_y, 20);
    }

    #[test]
    fn test_get_visible_region() {
        let mediator = BufferMediator::new(Rect::new(0, 0, 100, 100), 10, 20);
        let area = Rect::new(5, 15, 50, 50);
        let visible_region = mediator.get_visible_region(&area);
        assert_eq!(visible_region, Rect::new(10, 20, 45, 45));
    }

    #[test]
    fn test_map_to_screen_space() {
        let mediator = BufferMediator::new(Rect::new(0, 0, 100, 100), 10, 20);
        let area = Rect::new(15, 25, 50, 50);
        let screen_space = mediator.map_to_screen_space(&area);
        assert_eq!(screen_space, Rect::new(5, 5, 50, 50));
    }

    #[test]
    fn test_generate_inner() {
        let mediator = BufferMediator::new(Rect::new(0, 0, 100, 100), 10, 20);
        let area = Rect::new(5, 15, 50, 50);
        let inner_mediator = mediator.generate_inner(&area, 5, 10);
        assert_eq!(inner_mediator.area, Rect::new(0, 0, 45, 45));
        assert_eq!(inner_mediator.offset_x, 10);
        assert_eq!(inner_mediator.offset_y, 15);
    }
    #[test]
    fn test_generate_inner_upper_boundary() {
        let mediator = BufferMediator::new(Rect::new(0, 0, 100, 100), 10, 20);
        let area = Rect::new(70, 80, 50, 50);
        let inner_mediator = mediator.generate_inner(&area, 5, 10);
        assert_eq!(inner_mediator.area, Rect::new(60, 60, 40, 40));
        assert_eq!(inner_mediator.offset_x, 5);
        assert_eq!(inner_mediator.offset_y, 10);
    }
    #[test]
    fn test_size() {
        let mediator = BufferMediator::new(Rect::new(0, 0, 100, 100), 10, 20);
        assert_eq!(mediator.size(), Rect::new(0, 0, 100, 100));
    }
    #[test]
    fn write() {
        let area = Rect::new(10, 10, 60, 60);
        let mediator = BufferMediator::new(area, 10, 20);
        let mut buffer: VecBuffer<bool> = VecBuffer::new(100, 100);
        let area = Rect::new(0, 0, 100, 200);
        let data = vec![true; 20000];
        mediator.write(data.as_slice(), area, &mut buffer);
        for y in 0..100 {
            for x in 0..100 {
                assert_eq!(
                    (buffer[(x, y)], x, y),
                    ((x >= 10 && x < 70) && (y >= 10 && y < 70), x, y)
                )
            }
        }
    }
}
