use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::text::{Text, ToText};
use tui_input::Input;

pub struct TextInput {
    input: Input,
}

pub enum InputEvent {
    Ended,
    Cancelled,
    Continue,
}

impl TextInput {
    pub fn new() -> Self {
        Self {
            input: Input::new("".to_string()),
        }
    }

    pub fn to_text(&self) -> Text {
        self.input.to_text()
    }

    pub fn handle_event(&mut self, event: &Event) -> InputEvent {
        match event {
            Event::Key(k) if k.kind == KeyEventKind::Press => match k.code {
                KeyCode::Enter => {
                    return InputEvent::Ended;
                }
                KeyCode::Esc => {
                    return InputEvent::Cancelled;
                }
                KeyCode::Backspace => {
                    self.input.handle(tui_input::InputRequest::DeletePrevChar);
                }
                KeyCode::Char(char_to_insert) => {
                    self.input
                        .handle(tui_input::InputRequest::InsertChar(char_to_insert));
                }
                _ => {}
            },
            _ => {}
        }
        InputEvent::Continue
    }
}

impl ToString for TextInput {
    fn to_string(&self) -> String {
        self.input.to_string()
    }
}
