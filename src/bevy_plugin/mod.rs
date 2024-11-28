use std::io::stdout;

use crate::renderer::{
    buffer::VecBuffer,
    buffer_mediator::BufferMediator,
    core_widgetes::{border::Border, scrollbar::Scrollbar},
    painter::Painter,
    rect::Rect,
    terminal_writer::TerminalWriter,
    widget::Widget,
    BackgroundColor, ForegroundColor, Simble,
};
use bevy::{
    app::{App, Plugin, PostUpdate, PreUpdate, Startup},
    prelude::{Component, IntoSystemConfigs, Query, ResMut, Resource, SystemSet},
    text::Text,
};
use compact_str::ToCompactString;
use crossterm::terminal::size;
pub struct SmallTuiPlugin;

impl Plugin for SmallTuiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SmalltuiTerminal(TerminalWriter::new(30, 30, stdout())))
            .insert_resource(RenderQueue(Vec::new()))
            .insert_resource(ScreenSize {
                width: 5,
                height: 5,
            })
            .add_systems(Startup, setup)
            .add_systems(PreUpdate, resize_screen)
            .add_systems(
                PostUpdate,
                render_all.in_set(TuiRenerStages::ExcecuteRenders),
            );
    }
}
#[derive(SystemSet, Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum TuiRenerStages {
    QueueRenders,
    ExcecuteRenders,
}
#[derive(Resource)]
pub struct SmalltuiTerminal(
    pub TerminalWriter<VecBuffer<BackgroundColor>, VecBuffer<ForegroundColor>, VecBuffer<Simble>>,
);
pub struct RenderRequest {
    widget: Box<
        dyn Widget<
                TerminalWriter<
                    VecBuffer<BackgroundColor>,
                    VecBuffer<ForegroundColor>,
                    VecBuffer<Simble>,
                >,
            > + Send,
    >,
    depth: u16,
    area: Rect,
    scroll_x: u16,
    scroll_y: u16,
}
#[derive(Resource)]
pub struct RenderQueue(pub Vec<RenderRequest>);

#[derive(Component)]
pub struct UINode {
    pub area: Rect,
    pub z: u16,
}

#[derive(Component)]
pub struct ComputedUINode {
    depth: u16,
    area: Rect,
    scroll_x: u16,
    scroll_y: u16,
}

pub fn render_all(mut queue: ResMut<RenderQueue>, mut writer: ResMut<SmalltuiTerminal>) {
    let requests = &mut queue.0;
    let writer = &mut writer.as_mut().0;
    writer.background_fill(crossterm::style::Color::Cyan.into(), None);
    requests.sort_by(|a, b| a.depth.cmp(&b.depth).reverse());
    while let Some(r) = requests.pop() {
        let widget = r.widget.as_ref();
        writer.render_widget(widget, r.area, r.scroll_x, r.scroll_y);
    }
    writer.flush_frame().unwrap();
}

pub fn queue_renders(mut queue: ResMut<RenderQueue>, /* , q: Query<(&ComputedUINode, &Text)>*/) {
    //for (area, text) in &q {
    for i in 0..18 {
        queue.0.push(RenderRequest {
            widget: Box::new(Border::new(
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
            )),
            depth: 0,
            area: Rect::new(8 * (i * 6), 5 * (i / 6), 8, 5),
            scroll_x: i,
            scroll_y: i,
        });
    }
}
#[derive(Resource)]
pub struct ScreenSize {
    pub width: u16,
    pub height: u16,
}
pub fn resize_screen(mut writer: ResMut<SmalltuiTerminal>, mut sc: ResMut<ScreenSize>) {
    let (current_width, current_height) = size().unwrap();
    if (current_width, current_height) != (sc.width, sc.height) {
        sc.width = current_width;
        sc.height = current_height;
        writer.0.resize(current_width, current_height);
    }
}
pub fn setup(mut writer: ResMut<SmalltuiTerminal>) {
    writer.0.prepare_area();
}
