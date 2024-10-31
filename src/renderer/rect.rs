use std::cmp;

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
        let (xmin, xmax) = (self.x, self.x + self.width);
        let (ymin, ymax) = (self.y, self.y + self.height);
        let starting_x = cmp::min(cmp::max(xmin, other.x), xmax);
        let starting_y = cmp::min(cmp::max(ymin, other.y), ymax);
        let endpoint_x = cmp::max(cmp::min(xmax, other.x + other.width), xmin);
        let endpoint_y = cmp::max(cmp::min(ymax, other.y + other.height), ymin);
        Rect {
            x: starting_x,
            y: starting_y,
            width: endpoint_x - starting_x,
            height: endpoint_y - starting_y,
        }
    }
}
