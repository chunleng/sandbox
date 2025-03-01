use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    widgets::{Block, Paragraph, Widget},
};

use crate::control::input::TextInput;

pub struct InputPopup {
    title: String,
    pub input: TextInput,
}

impl InputPopup {
    pub fn new(title: String) -> Self {
        InputPopup {
            title,
            input: TextInput::new(),
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([Constraint::Length(3)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(50)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        let block = Block::bordered().title(self.title.to_string());

        Paragraph::new(self.input.to_text())
            .block(block)
            .render(area, buf);
    }
}
