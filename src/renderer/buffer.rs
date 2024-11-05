use crate::renderer::buffer_mediator::BufferMediator;
use crate::renderer::rect::Rect;
//use crate::renderer::buffer_delegate::BufferMediator;
use std::ops::{Index, IndexMut};
pub trait Buffer<T: Default + Sized + Clone>:
    Index<(u16, u16), Output = T> + IndexMut<(u16, u16), Output = T>
{
    fn new(width: u16, height: u16) -> Self;
    fn draw_line(&mut self, data: &[T], x: u16, y: u16, lenght: u16);
    fn get_mediator(&self, region: Option<Rect>) -> BufferMediator;
    fn area(&self) -> Rect;
}

struct VecBuffer<T: Default + Sized + Clone> {
    width: u16,
    height: u16,
    data: Vec<T>,
}
impl<T: Default + Sized + Clone> Buffer<T> for VecBuffer<T> {
    fn new(width: u16, height: u16) -> Self {
        VecBuffer::<T> {
            width: width,
            height: height,
            data: vec![T::default(); width as usize * height as usize],
        }
    }
    fn draw_line(&mut self, data: &[T], x: u16, y: u16, lenght: u16) {
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
}
impl<T: Default + Sized + Clone> Index<(u16, u16)> for VecBuffer<T> {
    fn index(&self, index: (u16, u16)) -> &Self::Output {
        &self.data[index.1 as usize * self.width as usize + index.1 as usize]
    }

    type Output = T;
}
impl<T: Default + Sized + Clone> IndexMut<(u16, u16)> for VecBuffer<T> {
    fn index_mut(&mut self, index: (u16, u16)) -> &mut Self::Output {
        &mut self.data[index.1 as usize * self.width as usize + index.1 as usize]
    }
}
