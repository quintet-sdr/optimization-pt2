use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Default)]
enum RunningState {
    #[default]
    Running,
    Done,
}

#[derive(Default)]
struct App {
    running_state: RunningState,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while matches!(self.running_state, RunningState::Running) {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
                self.exit()
            }
            _ => (),
        }
    }

    fn exit(&mut self) {
        self.running_state = RunningState::Done;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Introduction to Optimization / Programming Task 2 ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "←".blue().bold(),
            "/".into(),
            "h".blue().bold(),
            " Increment ".into(),
            "→".blue().bold(),
            "/".into(),
            "l".blue().bold(),
            " Quit ".into(),
            "esc".blue().bold(),
            "/".into(),
            "q ".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            52.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
