use std::cmp::{max, min};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let rect = Rect::new(10, 20, 30, 40);
        assert_eq!(
            rect,
            Rect {
                x: 10,
                y: 20,
                width: 30,
                height: 40
            }
        );
    }

    #[test]
    fn test_contains() {
        let rect1 = Rect::new(10, 20, 30, 40);
        let rect2 = Rect::new(15, 25, 10, 10);
        let rect3 = Rect::new(0, 0, 50, 50);
        assert!(rect1.contains(&rect2));
        assert!(!rect1.contains(&rect3));
    }

    #[test]
    fn test_offset() {
        let rect = Rect::new(10, 20, 30, 40);
        let offset_rect = rect.offset(5, 10);
        assert_eq!(
            offset_rect,
            Rect {
                x: 15,
                y: 30,
                width: 30,
                height: 40
            }
        );
    }

    #[test]
    fn test_crop() {
        let rect1 = Rect::new(10, 20, 30, 40);
        let rect2 = Rect::new(20, 30, 30, 30);
        let cropped_rect = rect1.crop(&rect2);
        assert_eq!(
            cropped_rect,
            Rect {
                x: 20,
                y: 30,
                width: 20,
                height: 30
            }
        );
    }

    #[test]
    fn test_bottom() {
        let rect = Rect::new(10, 20, 30, 40);
        assert_eq!(rect.bottom(), 60);
    }

    #[test]
    fn test_right() {
        let rect = Rect::new(10, 20, 30, 40);
        assert_eq!(rect.right(), 40);
    }

    #[test]
    fn test_left() {
        let rect = Rect::new(10, 20, 30, 40);
        assert_eq!(rect.left(), 10);
    }

    #[test]
    fn test_top() {
        let rect = Rect::new(10, 20, 30, 40);
        assert_eq!(rect.top(), 20);
    }
}
