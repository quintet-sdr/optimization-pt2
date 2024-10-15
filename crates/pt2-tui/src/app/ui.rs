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
    Frame,
};

use super::App;

impl App {
    pub(super) fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
