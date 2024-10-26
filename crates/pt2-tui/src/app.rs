use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::DefaultTerminal;

mod ui;

#[derive(Default)]
enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Default)]
pub struct App {
    running_state: RunningState,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while matches!(self.running_state, RunningState::Running) {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if matches!(key_event.kind, KeyEventKind::Press) => {
                self.handle_key_event(key_event);
            }
            _ => (),
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.exit(),
            KeyCode::Char('c') if matches!(key_event.modifiers, KeyModifiers::CONTROL) => {
                self.exit();
            }
            _ => (),
        }
    }

    fn exit(&mut self) {
        self.running_state = RunningState::Done;
    }
}
