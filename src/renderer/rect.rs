use std::cmp::{max, min};

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}
impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }
    pub fn contains(&self, other: &Rect) -> bool {
        return self.x <= other.x
            && self.y <= other.y
            && (self.x + self.width) >= (other.x + other.width)
            && (self.y + self.height) >= (other.y + other.height);
    }
    pub fn offset(&self, x: u16, y: u16) -> Rect {
        Rect::new(self.x + x, self.y + y, self.width, self.height)
    }
    pub fn crop(&self, other: &Rect) -> Rect {
        let x1 = max(self.x, other.x);
        let y1 = max(self.y, other.y);
        let x2 = min(self.right(), other.right());
        let y2 = min(self.bottom(), other.bottom());
        Self {
            x: x1,
            y: y1,
            width: x2.saturating_sub(x1),
            height: y2.saturating_sub(y1),
        }
    }
    pub fn bottom(&self) -> u16 {
        self.height + self.y
    }
    pub fn right(&self) -> u16 {
        self.width + self.x
    }
    pub fn left(&self) -> u16 {
        self.x
    }
    pub fn top(&self) -> u16 {
        self.y
    }
}
