use std::{
    error::Error,
    sync::{Arc, RwLock},
    thread::{self, sleep},
    time::Duration,
};

use component::{
    add::{AddComponent, AddEvent},
    delete::{DeleteComponent, DeleteEvent},
    update::{UpdateComponent, UpdateEvent},
};
use crossterm::event::{self, Event, KeyCode, poll};
use db::{Person, init_db};
use electric::{SyncOperation, sync};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    text::Text,
    widgets::{Block, Borders, List, Widget},
};

mod component;
mod control;
mod db;
mod electric;

fn main() -> Result<(), Box<dyn Error>> {
    init_db();
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();

    app_result
}

#[derive(PartialEq, Eq)]
enum AppMode {
    Main,
    Add,
    Delete,
    Update,
}

impl Default for AppMode {
    fn default() -> Self {
        Self::Main
    }
}

#[derive(Default)]
struct Components {
    add_component: AddComponent,
    delete_component: DeleteComponent,
    update_component: UpdateComponent,
}

#[derive(Default)]
struct App {
    exit: bool,
    persons: Arc<RwLock<Vec<Person>>>,
    logs: Arc<RwLock<Vec<Vec<SyncOperation>>>>,
    mode: AppMode,
    components: Components,
}

impl App {
    fn new() -> Self {
        Self::default()
    }

    fn spawn_updater(&self) {
        let persons = self.persons.clone();
        let logs = self.logs.clone();
        thread::spawn(move || {
            loop {
                let mut up_to_date = true;
                let sync = sync();
                {
                    let mut persons_guard = persons.write().unwrap();
                    sync.iter().for_each(|x| match x {
                        SyncOperation::Insert(y) => {
                            persons_guard.push(Person {
                                id: y.id,
                                name: y.name.to_owned(),
                                age: y.age,
                            });
                        }
                        SyncOperation::Delete(y) => {
                            if let Some(idx) = persons_guard.iter().position(|z| z.id == y.id) {
                                persons_guard.remove(idx);
                            }
                        }
                        SyncOperation::Update(y) => {
                            if let Some(idx) = persons_guard.iter().position(|z| z.id == y.id) {
                                if let Some(element) = persons_guard.get_mut(idx) {
                                    if let Some(name) = &y.name {
                                        element.name = name.to_owned();
                                    }
                                    if let Some(age) = &y.age {
                                        element.age = age.to_owned();
                                    }
                                }
                            }
                        }
                        SyncOperation::UpToDate => {
                            up_to_date = false;
                        }
                    })
                }
                {
                    let mut logs_guard = logs.write().unwrap();
                    if sync != [SyncOperation::UpToDate] {
                        logs_guard.push(sync);
                    }
                }

                if !up_to_date {
                    sleep(Duration::from_secs(1));
                }
            }
        });
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn Error>> {
        self.spawn_updater();
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
        if poll(Duration::from_secs(1))? {
            match &self.mode {
                AppMode::Main => match event::read()? {
                    Event::Key(k)
                        if k.code == KeyCode::Char('q') || k.code == KeyCode::Char('Q') =>
                    {
                        self.exit = true;
                    }
                    Event::Key(k)
                        if k.code == KeyCode::Char('a') || k.code == KeyCode::Char('A') =>
                    {
                        self.components.add_component = AddComponent::default();
                        self.mode = AppMode::Add;
                    }
                    Event::Key(k)
                        if k.code == KeyCode::Char('d') || k.code == KeyCode::Char('D') =>
                    {
                        self.components.delete_component = DeleteComponent::default();
                        self.mode = AppMode::Delete;
                    }
                    Event::Key(k)
                        if k.code == KeyCode::Char('u') || k.code == KeyCode::Char('U') =>
                    {
                        self.components.update_component = UpdateComponent::default();
                        self.mode = AppMode::Update;
                    }
                    _ => {}
                },
                AppMode::Add => {
                    if self.components.add_component.handle_event(&event::read()?)
                        == AddEvent::Ended
                    {
                        self.mode = AppMode::Main;
                    }
                }
                AppMode::Delete => {
                    if self
                        .components
                        .delete_component
                        .handle_event(&event::read()?)
                        == DeleteEvent::Ended
                    {
                        self.mode = AppMode::Main;
                    }
                }
                AppMode::Update => {
                    if self
                        .components
                        .update_component
                        .handle_event(&event::read()?)
                        == UpdateEvent::Ended
                    {
                        self.mode = AppMode::Main;
                    }
                }
            }
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let overall_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(7),
                Constraint::Length(1),
            ])
            .split(area);

        let p: Vec<String> = self
            .persons
            .read()
            .unwrap()
            .iter()
            .map(|x| match &x.age {
                Some(age) => format!("{}. {}, {} y.o.", x.id, x.name, age),
                _ => format!("{}. {}", x.id, x.name),
            })
            .collect();
        List::new(p)
            .block(Block::new().borders(Borders::ALL).title("Persons"))
            .render(overall_layout[0], buf);

        let l = self.logs.read().unwrap();
        let l: Vec<String> = l[l.len().saturating_sub(5)..]
            .iter()
            .map(|x| format!("{:?}", x))
            .collect();
        List::new(l)
            .block(Block::new().borders(Borders::ALL).title("Logs"))
            .render(overall_layout[1], buf);

        match &self.mode {
            AppMode::Main => {
                Text::raw(
                    "Press 'q' to quit, 'a' to add item, 'd' to delete item, 'u' to update item",
                )
                .render(overall_layout[2], buf);
            }
            AppMode::Add => {
                self.components.add_component.render(area, buf);
            }
            AppMode::Delete => {
                self.components.delete_component.render(area, buf);
            }
            AppMode::Update => {
                self.components.update_component.render(area, buf);
            }
        }
    }
}
