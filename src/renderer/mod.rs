pub mod buffer;
pub mod buffer_mediator;
pub mod core_widgetes;
pub mod painter;
pub mod rect;
pub mod terminal_writer;
pub mod widget;
use compact_str::CompactString;
use crossterm::style::Color;
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct BackgroundColor(Color);

impl Default for BackgroundColor {
    fn default() -> Self {
        Self(Color::Black)
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ForegroundColor(Color);
impl Default for ForegroundColor {
    fn default() -> Self {
        Self(Color::White)
    }
}
#[derive(Clone, PartialEq, Eq)]
pub struct Simble(CompactString);
impl Default for Simble {
    fn default() -> Self {
        Simble(CompactString::const_new(" "))
    }
}

impl Into<ForegroundColor> for Color {
    fn into(self) -> ForegroundColor {
        ForegroundColor(self)
    }
}
impl Into<BackgroundColor> for Color {
    fn into(self) -> BackgroundColor {
        BackgroundColor(self)
    }
}
impl Into<Simble> for CompactString {
    fn into(self) -> Simble {
        Simble(self)
    }
}
