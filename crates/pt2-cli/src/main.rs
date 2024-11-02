use std::{io, panic};

use color_eyre::Result;
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

mod cli;
mod config;

fn main() -> Result<()> {
    color_eyre::install()?;

    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
    panic::set_hook(Box::new(|_| {
        crossterm::execute!(io::stdout(), Clear(ClearType::All)).unwrap();
        crossterm::execute!(io::stdout(), LeaveAlternateScreen).unwrap();
    }));

    let result = cli::run();
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
    result
}
