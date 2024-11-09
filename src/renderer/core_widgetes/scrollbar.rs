use crate::renderer::{
    core_widgetes::scrollbar, painter::Painter, rect::Rect, widget::Widget, BackgroundColor,
    Direction, ForegroundColor, Simble,
};

pub struct Scrollbar {
    lenght: u16,
    grabbable_lenght: u16,
    position: u16,
    bar_simble: Simble,
    grabbable_simble: Simble,
    less_arrow_simble: Simble,
    more_arrow_simble: Simble,
    direction: Direction,
    background_color: Option<BackgroundColor>,
    foreground_color: Option<ForegroundColor>,
    background_color_grabbable: Option<BackgroundColor>,
    foreground_color_grabbable: Option<ForegroundColor>,
    background_color_arrow: Option<BackgroundColor>,
    foreground_color_arrow: Option<ForegroundColor>,
}

impl Scrollbar {
    pub fn new(
        lenght: u16,
        grabbable_lenght: u16,
        position: u16,
        bar_simble: Simble,
        grabbable_simble: Simble,
        less_arrow_simble: Simble,
        more_arrow_simble: Simble,
        direction: Direction,
        background_color: Option<BackgroundColor>,
        foreground_color: Option<ForegroundColor>,
        background_color_grabbable: Option<BackgroundColor>,
        foreground_color_grabbable: Option<ForegroundColor>,
        background_color_arrow: Option<BackgroundColor>,
        foreground_color_arrow: Option<ForegroundColor>,
    ) -> Self {
        if lenght < 2 {
            panic!("scrollbars require at least a lenght of 2")
        }
        if grabbable_lenght + position > lenght {
            panic!("grabbable element should fit inside the lenght of the bar")
        }
        Self {
            lenght,
            grabbable_lenght,
            position,
            bar_simble,
            grabbable_simble,
            less_arrow_simble,
            more_arrow_simble,
            direction,
            background_color,
            foreground_color,
            background_color_grabbable,
            foreground_color_grabbable,
            background_color_arrow,
            foreground_color_arrow,
        }
    }
}
impl<'a, P: Painter<'a>> Widget<'a, P> for Scrollbar {
    fn render_widget<'b>(&self, painter: &'b mut P) {
        let mut simbles = vec![self.bar_simble.clone(); self.lenght as usize];
        let grabbable = vec![self.grabbable_simble.clone(); self.grabbable_lenght as usize];
        let scrollbar_slice =
            &mut simbles[self.position as usize..(self.position + self.grabbable_lenght) as usize];
        let grabbable_slice = grabbable.as_slice();
        scrollbar_slice.clone_from_slice(grabbable_slice);
        simbles[0] = self.less_arrow_simble.clone();
        simbles[(self.lenght - 1) as usize] = self.more_arrow_simble.clone();
        let horizontal = Rect::new(0, 0, simbles.len() as u16, 1);
        let vertical = Rect::new(0, 0, simbles.len() as u16, 1);
        match self.direction {
            Direction::UP => {
                painter.write_simbles(&simbles[simbles.len()..0], vertical);
            }
            Direction::Down => {
                painter.write_simbles(&simbles.as_slice(), vertical);
            }
            Direction::Left => {
                painter.write_simbles(&simbles[simbles.len()..0], horizontal);
            }
            Direction::Right => {
                painter.write_simbles(&simbles.as_slice(), horizontal);
            }
        }
    }
}
