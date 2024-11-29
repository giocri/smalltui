use crate::renderer::buffer_mediator::BufferMediator;
use crate::renderer::rect::Rect;
//use crate::renderer::buffer_delegate::BufferMediator;
use std::ops::{Index, IndexMut};
pub trait Buffer<T: Default + Sized + Clone + Send + Sync>:
    Index<(u16, u16), Output = T> + IndexMut<(u16, u16), Output = T> + Send + Sync
{
    fn new(width: u16, height: u16) -> Self;
    fn draw_line(&mut self, data: &[T], x: u16, y: u16, lenght: u16);
    fn get_mediator(&self, region: Option<Rect>) -> BufferMediator;
    fn area(&self) -> Rect;
    fn reset(&mut self);
    fn resize(&mut self, width: u16, height: u16);
}

pub struct VecBuffer<T: Default + Sized + Clone> {
    width: u16,
    height: u16,
    data: Vec<T>,
}
impl<T: Default + Sized + Clone + Send + Sync> Buffer<T> for VecBuffer<T> {
    fn new(width: u16, height: u16) -> Self {
        VecBuffer::<T> {
            width: width,
            height: height,
            data: vec![T::default(); width as usize * height as usize],
        }
    }
    fn draw_line(&mut self, data: &[T], x: u16, y: u16, lenght: u16) {
        if lenght + x > self.width {
            panic!("x overflow")
        }
        if y >= self.height {
            panic!("y overflow")
        }
        let start = (y * self.width + x) as usize;

        let buffer_section = &mut self.data[start..start + lenght as usize];
        buffer_section.clone_from_slice(&data[0..lenght as usize]);
    }
    fn get_mediator(&self, region: Option<Rect>) -> BufferMediator {
        let buffer_area = self.area();
        let mediator_region = if let Some(r) = region {
            buffer_area.crop(&r)
        } else {
            buffer_area
        };

        BufferMediator::new(mediator_region, 0, 0)
    }
    fn area(&self) -> Rect {
        Rect::new(0, 0, self.width, self.height)
    }

    fn reset(&mut self) {
        self.resize(self.width, self.height);
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.data = vec![T::default(); width as usize * height as usize]
    }
}
impl<T: Default + Sized + Clone> Index<(u16, u16)> for VecBuffer<T> {
    fn index(&self, index: (u16, u16)) -> &Self::Output {
        &self.data[index.1 as usize * self.width as usize + index.0 as usize]
    }

    type Output = T;
}
impl<T: Default + Sized + Clone> IndexMut<(u16, u16)> for VecBuffer<T> {
    fn index_mut(&mut self, index: (u16, u16)) -> &mut Self::Output {
        &mut self.data[index.1 as usize * self.width as usize + index.0 as usize]
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_new() {
        let buffer: VecBuffer<u8> = VecBuffer::new(10, 20);
        assert_eq!(buffer.width, 10);
        assert_eq!(buffer.height, 20);
        assert_eq!(buffer.data.len(), 200);
    }

    #[test]
    fn test_draw_line() {
        let mut buffer: VecBuffer<u8> = VecBuffer::new(10, 20);
        let data = vec![1, 2, 3, 4, 5];
        buffer.draw_line(&data, 2, 5, 5);
        assert_eq!(&buffer.data[52..57], &[1, 2, 3, 4, 5]);
    }

    #[test]
    #[should_panic]
    fn test_draw_line_x_overflow() {
        let mut buffer: VecBuffer<u8> = VecBuffer::new(10, 20);
        buffer.draw_line(&vec![1, 2, 3, 4, 5], 8, 5, 5);
    }

    #[test]
    #[should_panic]
    fn test_draw_line_y_overflow() {
        let mut buffer: VecBuffer<u8> = VecBuffer::new(10, 20);
        buffer.draw_line(&vec![1, 2, 3, 4, 5], 2, 21, 5);
    }

    #[test]
    fn test_get_mediator() {
        let buffer: VecBuffer<u8> = VecBuffer::new(10, 20);
        let mediator = buffer.get_mediator(Some(Rect::new(1, 1, 5, 5)));
        assert_eq!(BufferMediator::new(Rect::new(1, 1, 5, 5), 0, 0), mediator);
    }

    #[test]
    fn test_area() {
        let buffer: VecBuffer<u8> = VecBuffer::new(10, 20);
        assert_eq!(buffer.area(), Rect::new(0, 0, 10, 20));
    }

    #[test]
    fn test_reset() {
        let mut buffer: VecBuffer<u8> = VecBuffer::new(10, 20);
        buffer.draw_line(&vec![1, 2, 3, 4, 5], 2, 5, 5);
        buffer.reset();
        assert_eq!(buffer.data, vec![0; 200]);
    }

    #[test]
    fn test_resize() {
        let mut buffer: VecBuffer<u8> = VecBuffer::new(10, 20);
        buffer.resize(20, 30);
        assert_eq!(buffer.width, 20);
        assert_eq!(buffer.height, 30);
        assert_eq!(buffer.data.len(), 600);
    }

    #[test]
    fn test_index() {
        let buffer: VecBuffer<u8> = VecBuffer::new(10, 20);
        assert_eq!(buffer[(2, 3)], 0);
    }

    #[test]
    fn test_index_mut() {
        let mut buffer: VecBuffer<u8> = VecBuffer::new(10, 20);
        buffer[(2, 3)] = 5;
        assert_eq!(buffer[(2, 3)], 5);
    }
}
