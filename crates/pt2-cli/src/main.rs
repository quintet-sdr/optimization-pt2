use std::io;

use color_eyre::{eyre::Context, Result};
use crossterm::terminal::{Clear, ClearType};

mod config;

fn main() -> Result<()> {
    const ALPHA_1: f64 = 0.5;
    const ALPHA_2: f64 = 0.9;

    color_eyre::install()?;

    let tests = config::read_tests().wrap_err("tests.json not found")?;

    loop {
        crossterm::execute!(io::stdout(), Clear(ClearType::All))?;

        let Some(test) = inquire::Select::new("Select a test:", tests.clone())
            .with_vim_mode(true)
            .prompt_skippable()?
        else {
            break;
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

        if !inquire::Confirm::new("Next test?").prompt()? {
            break;
        };
    }

    Ok(())
}
