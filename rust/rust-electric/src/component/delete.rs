use crossterm::event::Event;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
    control::{input::InputEvent, popup::InputPopup},
    db::delete_person,
};

pub struct DeleteState {
    id_input_popup: InputPopup,
}

impl Default for DeleteState {
    fn default() -> Self {
        Self {
            id_input_popup: InputPopup::new("Please input the ID to delete".to_string()),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum DeleteEvent {
    Continue,
    Ended,
}

#[derive(Default)]
pub struct DeleteComponent {
    state: Box<DeleteState>,
}

impl DeleteComponent {
    pub fn handle_event(&mut self, event: &Event) -> DeleteEvent {
        match self.state.id_input_popup.input.handle_event(&event) {
            InputEvent::Continue => return DeleteEvent::Continue,
            InputEvent::Cancelled => return DeleteEvent::Ended,
            InputEvent::Ended => {
                if let Some(id) = self.state.id_input_popup.input.to_string().parse().ok() {
                    delete_person(id);
                }
                return DeleteEvent::Ended;
            }
        };
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        self.state.id_input_popup.render(area, buf);
    }
}
