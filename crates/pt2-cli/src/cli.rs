use std::io;

use color_eyre::{eyre::Context, Result};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};

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

fn prompt(tests: Vec<Test>) -> Result<Next> {
    for _ in 0..crossterm::terminal::size()?.1 {
        println!();
    }

    let Some(test) = inquire::Select::new("Select a test:", tests)
        .with_vim_mode(true)
        .prompt_skippable()?
    else {
        return Ok(Next::Break);
    };

    let Some(alpha) = inquire::Select::new("Select an alpha:", vec![0.5, 0.9])
        .with_vim_mode(true)
        .prompt_skippable()?
    else {
        return Ok(Next::Break);
    };

    'a: {
        let Ok((lpp, iterations)) = pt2_core::interior_point(
            test.objective_function.clone(),
            &test.constraints,
            test.initial_point.clone(),
            test.eps,
            alpha,
        ) else {
            println!("The method is not applicable.");
            break 'a;
        };

        println!("Epsilon: {} ({:.eps$})", test.eps, lpp.eps, eps = test.eps);

        println!(
            "Objective function: {:.eps$?}",
            lpp.c.iter().collect::<Box<[_]>>(),
            eps = test.eps,
        );
        println!(
            "Initial point: {:.eps$?}",
            lpp.x.iter().collect::<Box<[_]>>(),
            eps = test.eps,
        );
        println!("Constraints:{:.eps$}", lpp.big_a, eps = test.eps);

        let last = iterations.last().unwrap();

        let Ok(result) = last else {
            println!("The problem doesn't have a solution.");
            break 'a;
        };

        println!("Result:");
        println!("Maximum: {:.eps$}", result.max, eps = test.eps);
        println!(
            "Decision variables: {:.eps$?}",
            result.decision_variables.iter().collect::<Box<[_]>>(),
            eps = test.eps
        );
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
