pub mod buffer;
pub mod buffer_mediator;
pub mod painter;
pub mod rect;
use compact_str::CompactString;
#[derive(Clone, Copy)]
pub enum Direction {
    UP,
    Down,
    Left,
    Right,
}
#[derive(Clone, Copy)]
pub enum BoxSide {
    UP,
    Down,
    Left,
    Right,
}
#[derive(Default, Clone, Copy)]
pub enum BackgroundColor {
    #[default]
    Black,
    White,
}
#[derive(Default, Clone, Copy)]
pub enum ForegroundColor {
    #[default]
    White,
    Black,
}
#[derive(Clone)]
pub struct Simble(CompactString);
impl Default for Simble {
    fn default() -> Self {
        Simble(CompactString::const_new(" "))
    }
}
