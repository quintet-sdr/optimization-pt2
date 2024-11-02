use std::io;

use color_eyre::{eyre::Context, Result};
use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

use crate::config::{self, Test};

pub fn run() -> Result<()> {
    let tests = config::read_tests().wrap_err("tests.json not found")?;
    while matches!(prompt(tests.clone())?, Next::Continue) {}
    Ok(())
}

enum Next {
    Continue,
    Break,
}

fn clear() -> io::Result<()> {
    crossterm::execute!(io::stdout(), Clear(ClearType::All))
}

fn prompt(tests: Vec<Test>) -> Result<Next> {
    const ALPHA_1: f64 = 0.5;
    const ALPHA_2: f64 = 0.9;

    clear()?;

    let Some(test) = inquire::Select::new("Select a test:", tests)
        .with_vim_mode(true)
        .prompt_skippable()?
    else {
        return Ok(Next::Break);
    };

    for alpha in [ALPHA_1, ALPHA_2] {
        println!("alpha: {alpha:.eps$}", eps = test.eps);

        let iterations = match pt2_core::interior_point(
            test.objective_function.clone(),
            &test.constraints,
            test.initial_point.clone(),
            test.eps,
            alpha,
        ) {
            Ok(it) => it,
            Err(err) => {
                println!("{err}");
                continue;
            }
        };

        let last = iterations.last().unwrap();

        let result = match last {
            Ok(it) => it,
            Err(err) => {
                println!("{err}");
                continue;
            }
        };

        println!("max: {:.eps$}", result.max, eps = test.eps);
        println!(
            "x:{:.eps$}",
            result.decision_variables.transpose(),
            eps = test.eps
        );
        println!();
    }

    let Some(next) = inquire::Confirm::new("Next test?").prompt_skippable()? else {
        return Ok(Next::Break);
    };
    if !next {
        return Ok(Next::Break);
    };

    Ok(Next::Continue)
}

pub fn enter_alternate_screen() -> io::Result<()> {
    crossterm::execute!(io::stdout(), EnterAlternateScreen)
}

pub fn leave_alternate_screen() -> io::Result<()> {
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)
}
