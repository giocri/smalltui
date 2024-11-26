use compact_str::ToCompactString;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    style::Color,
    terminal::size,
};
use smalltui::renderer::{
    buffer::VecBuffer,
    core_widgetes::{border::Border, scrollbar::Scrollbar},
    painter::{Painter, TextPainer},
    rect::Rect,
    terminal_writer::TerminalWriter,
    BackgroundColor, Direction, ForegroundColor, Simble,
};
use std::{env, io::stdout};
use wrapper::Wrappper;
mod wrapper;
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
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
        let area = a.area();
        //eprintln!("buffer size:{:?}", area);
        a.background_fill(Color::DarkBlue.into(), None);
        a.foreground_fill(Color::Green.into(), None);
        //p.simble_fill("A".to_compact_string().into(), None);
        a.background_fill(Color::Red.into(), Some(area.crop(&area.offset(10, 10))));
        a.foreground_fill(Color::Cyan.into(), Some(area.crop(&area.offset(20, 5))));
        a.background_fill(Color::Black.into(), Some(area.crop(&area.offset(15, 15))));
        /*p.simble_fill(
            "@".to_compact_string().into(),
            Some(area.crop(&area.offset(20, 5))),
        );*/
        a.write_paragraph(
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
            Direction::UP,
            Some(Color::DarkGreen.into()),
            Some(Color::White.into()),
            Some(Color::White.into()),
            Some(Color::Black.into()),
            Some(Color::DarkRed.into()),
            Some(Color::White.into()),
        );
        a.render_widget(&s, area.offset(20, 25), 0, 0);
        let b = Border::new(
            '#'.to_compact_string().into(),
            '#'.to_compact_string().into(),
            '|'.to_compact_string().into(),
            '|'.to_compact_string().into(),
            '-'.to_compact_string().into(),
            '-'.to_compact_string().into(),
            '#'.to_compact_string().into(),
            '#'.to_compact_string().into(),
            8,
            5,
        );
        let control = Border::new(
            '#'.to_compact_string().into(),
            '#'.to_compact_string().into(),
            '|'.to_compact_string().into(),
            '|'.to_compact_string().into(),
            '-'.to_compact_string().into(),
            '-'.to_compact_string().into(),
            '#'.to_compact_string().into(),
            '#'.to_compact_string().into(),
            10,
            7,
        );
        let wrapperd_b = Wrappper::new(Border::new(
            '#'.to_compact_string().into(),
            '#'.to_compact_string().into(),
            '|'.to_compact_string().into(),
            '|'.to_compact_string().into(),
            '-'.to_compact_string().into(),
            '-'.to_compact_string().into(),
            '#'.to_compact_string().into(),
            '#'.to_compact_string().into(),
            8,
            5,
        ));
        for i in 0..18 {
            a.render_widget(
                &b,
                Rect::new(1 + 11 * (i % 6), 1 + 8 * (i / 6), 8, 5),
                i % 6,
                i / 6,
            );
            a.render_widget(
                &wrapperd_b,
                Rect::new(81 + 11 * (i % 6), 1 + 8 * (i / 6), 8, 5),
                i % 6,
                i / 6,
            );
            a.render_widget(
                &control,
                Rect::new(80 + 11 * (i % 6), 0 + 8 * (i / 6), 10, 7),
                0,
                0,
            );
            a.render_widget(
                &control,
                Rect::new(0 + 11 * (i % 6), 0 + 8 * (i / 6), 10, 7),
                0,
                0,
            );
        }

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
