use crossterm::event::Event;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
    control::{input::InputEvent, popup::InputPopup},
    db::{UpdatePerson, update_person},
};

pub struct UpdateState {
    id_input_popup: InputPopup,
    name_input_popup: InputPopup,
    age_input_popup: InputPopup,
    pub step: UpdateStep,
}

impl Default for UpdateState {
    fn default() -> Self {
        Self {
            id_input_popup: InputPopup::new("Please input the ID:".to_string()),
            name_input_popup: InputPopup::new("Please input the name:".to_string()),
            age_input_popup: InputPopup::new("Please input the age:".to_string()),
            step: UpdateStep::default(),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum UpdateStep {
    Id,
    Name,
    Age,
}

impl Default for UpdateStep {
    fn default() -> Self {
        Self::Id
    }
}

#[derive(Default)]
pub struct UpdateComponent {
    state: Box<UpdateState>,
}

#[derive(PartialEq, Eq)]
pub enum UpdateEvent {
    Continue,
    Ended,
}

impl UpdateComponent {
    pub fn handle_event(&mut self, event: &Event) -> UpdateEvent {
        match self.state.step {
            UpdateStep::Id => {
                match self.state.id_input_popup.input.handle_event(&event) {
                    InputEvent::Continue => return UpdateEvent::Continue,
                    InputEvent::Cancelled => return UpdateEvent::Ended,
                    InputEvent::Ended => {
                        self.state.step = UpdateStep::Name;
                        return UpdateEvent::Continue;
                    }
                };
            }
            UpdateStep::Name => {
                match self.state.name_input_popup.input.handle_event(&event) {
                    InputEvent::Continue => return UpdateEvent::Continue,
                    InputEvent::Cancelled => return UpdateEvent::Ended,
                    InputEvent::Ended => {
                        self.state.step = UpdateStep::Age;
                        return UpdateEvent::Continue;
                    }
                };
            }
            UpdateStep::Age => match self.state.age_input_popup.input.handle_event(&event) {
                InputEvent::Continue => return UpdateEvent::Continue,
                InputEvent::Cancelled => {
                    self.state.step = UpdateStep::Name;
                    return UpdateEvent::Continue;
                }
                InputEvent::Ended => {
                    if let Some(id) = self.state.id_input_popup.input.to_string().parse().ok() {
                        let name = match self.state.name_input_popup.input.to_string() {
                            s if s == "" => None,
                            s => Some(s.to_string()),
                        };
                        let age = match self.state.age_input_popup.input.to_string() {
                            s if s == "" => None,
                            s => Some(s.parse().ok()),
                        };
                        update_person(id, UpdatePerson { name, age });
                    }
                    UpdateEvent::Ended
                }
            },
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        match self.state.step {
            UpdateStep::Id => {
                self.state.id_input_popup.render(area, buf);
            }
            UpdateStep::Name => {
                self.state.name_input_popup.render(area, buf);
            }
            UpdateStep::Age => {
                self.state.age_input_popup.render(area, buf);
            }
        }
    }
}
