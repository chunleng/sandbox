use crossterm::event::Event;
use ratatui::{buffer::Buffer, layout::Rect};

use crate::{
    control::{input::InputEvent, popup::InputPopup},
    db::{NewPerson, add_person},
};

pub struct AddState {
    name_input_popup: InputPopup,
    age_input_popup: InputPopup,
    pub step: AddStep,
}

impl Default for AddState {
    fn default() -> Self {
        Self {
            name_input_popup: InputPopup::new("Please input the name:".to_string()),
            age_input_popup: InputPopup::new("Please input the age:".to_string()),
            step: AddStep::default(),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum AddStep {
    Name,
    Age,
}

impl Default for AddStep {
    fn default() -> Self {
        Self::Name
    }
}

#[derive(Default)]
pub struct AddComponent {
    state: Box<AddState>,
}

#[derive(PartialEq, Eq)]
pub enum AddEvent {
    Continue,
    Ended,
}

impl AddComponent {
    pub fn handle_event(&mut self, event: &Event) -> AddEvent {
        match self.state.step {
            AddStep::Name => {
                match self.state.name_input_popup.input.handle_event(&event) {
                    InputEvent::Continue => return AddEvent::Continue,
                    InputEvent::Cancelled => return AddEvent::Ended,
                    InputEvent::Ended => {
                        self.state.step = AddStep::Age;
                        return AddEvent::Continue;
                    }
                };
            }
            AddStep::Age => {
                match self.state.age_input_popup.input.handle_event(&event) {
                    InputEvent::Continue => return AddEvent::Continue,
                    InputEvent::Cancelled => {
                        self.state.step = AddStep::Name;
                        return AddEvent::Continue;
                    }
                    InputEvent::Ended => {
                        add_person(NewPerson {
                            name: self.state.name_input_popup.input.to_string(),
                            age: self.state.age_input_popup.input.to_string().parse().ok(),
                        });
                        return AddEvent::Ended;
                    }
                };
            }
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        match self.state.step {
            AddStep::Name => {
                self.state.name_input_popup.render(area, buf);
            }
            AddStep::Age => {
                self.state.age_input_popup.render(area, buf);
            }
        }
    }
}
