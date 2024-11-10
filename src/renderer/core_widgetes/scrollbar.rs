use crate::renderer::{
    painter::Painter, rect::Rect, widget::Widget, BackgroundColor, Direction, ForegroundColor,
    Simble,
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
    fn handle_direction(&self, start: u16, lenght: u16) -> (Rect, bool) {
        match self.direction {
            Direction::UP => (
                Rect::new(0, self.lenght - (start + lenght - 1), 1, lenght),
                true,
            ),
            Direction::Down => (Rect::new(0, start, 1, lenght), false),
            Direction::Left => (
                Rect::new(self.lenght - (start + lenght - 1), 0, lenght, 1),
                true,
            ),
            Direction::Right => (Rect::new(start, 0, lenght, 1), false),
        }
    }
}
impl<P: Painter> Widget<P> for Scrollbar {
    fn render_widget<'b>(&self, painter: &'b mut P) {
        let mut simbles = vec![self.bar_simble.clone(); self.lenght as usize];
        let grabbable = vec![self.grabbable_simble.clone(); self.grabbable_lenght as usize];
        let scrollbar_slice =
            &mut simbles[self.position as usize..(self.position + self.grabbable_lenght) as usize];
        let grabbable_slice = grabbable.as_slice();
        scrollbar_slice.clone_from_slice(grabbable_slice);
        simbles[0] = self.less_arrow_simble.clone();
        simbles[(self.lenght - 1) as usize] = self.more_arrow_simble.clone();

        let s = self.handle_direction(0, self.lenght);
        if s.1 {
            simbles.reverse();
        }
        painter.write_simbles(&simbles, s.0);

        if let Some(color) = self.background_color {
            let t = vec![color; self.lenght as usize];
            let bc = self.handle_direction(0, self.lenght);
            painter.write_background_color(&t, bc.0);
        }
        if let Some(color) = self.foreground_color {
            let t = vec![color; self.lenght as usize];
            let fc = self.handle_direction(0, self.lenght);
            painter.write_foreground_color(&t, fc.0);
        }
        if let Some(color) = self.background_color_grabbable {
            let mut t = vec![color; self.grabbable_lenght as usize];
            let bc = self.handle_direction(self.position, self.grabbable_lenght);
            if bc.1 {
                t.reverse();
            }
            painter.write_background_color(&t, bc.0);
        }
        if let Some(color) = self.foreground_color_grabbable {
            let mut t = vec![color; self.grabbable_lenght as usize];
            let fc = self.handle_direction(self.position, self.grabbable_lenght);
            if fc.1 {
                t.reverse();
            }
            painter.write_foreground_color(&t, fc.0);
        }
        if let Some(color) = self.background_color_arrow {
            let t = vec![color; 1];
            let bc = self.handle_direction(0, 1);
            painter.write_background_color(&t, bc.0);
            let bc = self.handle_direction(self.lenght - 1, 1);
            painter.write_background_color(&t, bc.0);
        }
        if let Some(color) = self.foreground_color_arrow {
            let t = vec![color; 1];
            let fc = self.handle_direction(0, 1);
            painter.write_foreground_color(&t, fc.0);
            let fc = self.handle_direction(self.lenght - 1, 1);
            painter.write_foreground_color(&t, fc.0);
        }
    }
}
