use std::io;
use tui::{Terminal};
use crossterm::{
    event::{self, Event, KeyCode},
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment},
    widgets::{Block, Borders},
    Frame,
    style::{Color},

};
use tui::widgets::canvas::{Canvas, Rectangle};
use std::cmp::min;

// https://github.com/fdehau/tui-rs/blob/v0.18.0/examples/canvas.rs
/// App holds the state of the application
pub struct App {
    x: i32,
    y: i32,
    shape: Rectangle,
}

impl App {
    pub fn new() -> App {
        App {
            x: 0,
            y: 0,
            shape: Rectangle {
                x: 10.0,
                y: 30.0,
                width: 4.0,
                height: 8.0,
                color: Color::Yellow,
            },
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    // define the layout chunks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(4)
        .constraints(
            [
                Constraint::Percentage(70),
                Constraint::Percentage(30),
            ].as_ref()
        )
        .split(f.size());

    // draw and render the tetris well
    let mut block = Canvas::default().block(Block::default()
        .borders(Borders::ALL).title("Tetris (q to quit)")
        .title_alignment(Alignment::Center))
        .paint(|ctx| {
            ctx.draw(&app.shape);
        })
        .x_bounds([10.0, 110.0])
        .y_bounds([10.0, 110.0]);
    f.render_widget(block, chunks[0]);

    // draw and render the information pane
    let block = Block::default().title("Info")
        .title_alignment(Alignment::Center).borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {

        app.shape.y -= 5.0;
        // update the terminal
        terminal.draw(|f| {
            ui(f, &app);
        })?;


        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => {
                    // quit
                    return Ok(());
                }
                KeyCode::Down => {
                    app.shape.y -= min(5, 0) as f64;
                }
                KeyCode::Up => {
                    app.shape.y += 5.0;
                }
                KeyCode::Right => {
                    app.shape.x += 2.0;
                }
                KeyCode::Left => {
                    app.shape.x -= min(2, 0) as f64;
                }
                _ => {}
            }
        }
    }
}
