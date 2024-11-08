use compact_str::ToCompactString;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    style::Color,
    terminal::size,
};
use smalltui::renderer::{
    buffer::{Buffer, VecBuffer},
    core_widgetes::scrollbar::Scrollbar,
    painter::{simple_painter::SimplePainter, Painter, TextPainer},
    terminal_writer::TerminalWriter,
    widget::Widget,
    BackgroundColor, Direction, ForegroundColor, Simble,
};
use std::io::stdout;

fn main() {
    let stdout = stdout();
    let (mut width, mut height) = size().unwrap();
    let mut a: TerminalWriter<
        VecBuffer<BackgroundColor>,
        VecBuffer<ForegroundColor>,
        VecBuffer<Simble>,
    > = TerminalWriter::new(width, height, stdout);
    a.prepare_area();
    //let mut counter = 0;
    loop {
        let (current_width, current_height) = size().unwrap();
        if (current_width, current_height) != (width, height) {
            width = current_width;
            height = current_height;
            a.resize(width, height);
        }
        let buffs = a.buffers();
        let mediator = buffs.0.get_mediator(None);
        let area = buffs.0.area();
        let mut p = SimplePainter::new(buffs.0, buffs.1, buffs.2, mediator);
        p.background_fill(Color::DarkBlue.into(), None);
        p.foreground_fill(Color::Green.into(), None);
        //p.simble_fill("A".to_compact_string().into(), None);
        p.background_fill(Color::Red.into(), Some(area.crop(&area.offset(10, 10))));
        p.foreground_fill(Color::Cyan.into(), Some(area.crop(&area.offset(20, 5))));
        p.background_fill(Color::Black.into(), Some(area.crop(&area.offset(15, 15))));
        /*p.simble_fill(
            "@".to_compact_string().into(),
            Some(area.crop(&area.offset(20, 5))),
        );*/
        p.write_paragraph(
            "+----------+
|          |
|    MY    |
|   TUI    |
|  WORKS   |
|          |
+----------+",
            18,
            18,
            Some(42),
        );
        let s = Scrollbar::new(
            24,
            8,
            5,
            '-'.to_compact_string().into(),
            '#'.to_compact_string().into(),
            '<'.to_compact_string().into(),
            '>'.to_compact_string().into(),
            Direction::Right,
            None,
            None,
            None,
            None,
            None,
            None,
        );
        s.render_widget(&mut (p.delegate_painter(&area.offset(20, 25), 0, 0)));
        a.flush_frame().unwrap();
        match read_char().unwrap() {
            Some('q') => {
                break;
            }
            _ => {}
        };
    }
    a.clear();
}
pub fn read_char() -> std::io::Result<Option<char>> {
    if let Ok(Event::Key(KeyEvent {
        code: KeyCode::Char(c),
        kind: KeyEventKind::Press,
        modifiers: _,
        state: _,
    })) = event::read()
    {
        return Ok(Some(c));
    } else {
        return Ok(None);
    }
}
