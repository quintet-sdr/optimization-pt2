use std::panic;

use color_eyre::Result;

mod cli;
mod config;

fn main() -> Result<()> {
    color_eyre::install()?;

    cli::enter_alternate_screen()?;
    panic::set_hook(Box::new(|_| {
        cli::enter_alternate_screen().unwrap();
        cli::leave_alternate_screen().unwrap();
    }));

    let result = cli::run();

    cli::leave_alternate_screen()?;
    result
}
