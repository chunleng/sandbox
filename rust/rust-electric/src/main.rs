use std::error::Error;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();

    app_result
}

struct App {
    exit: bool,
}

impl App {
    fn new() -> Self {
        Self { exit: false }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_event()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_event(&mut self) -> Result<(), Box<dyn Error>> {
        match event::read()? {
            Event::Key(k) if k.code == KeyCode::Char('q') || k.code == KeyCode::Char('Q') => {
                self.exit = true;
            }
            _ => {}
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let overall_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(1)])
            .split(area);
        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(overall_layout[0]);

        Paragraph::new("Shows items")
            .block(Block::new().borders(Borders::ALL))
            .render(content_layout[0], buf);
        Paragraph::new("Shows log")
            .block(Block::new().borders(Borders::ALL))
            .render(content_layout[1], buf);
        Text::raw("Press 'Q' to quit").render(overall_layout[1], buf);
    }
}
