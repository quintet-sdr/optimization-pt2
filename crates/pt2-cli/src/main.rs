use color_eyre::{eyre::Context, Result};

mod config;

fn main() -> Result<()> {
    const ALPHA_1: f64 = 0.5;
    const ALPHA_2: f64 = 0.9;

    color_eyre::install()?;

    for test in config::read_tests().wrap_err("tests.json not found")? {
        for alpha in [ALPHA_1, ALPHA_2] {
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

            println!("alpha: {alpha:.eps$}", eps = test.eps);
            println!("max: {:.eps$}", result.max, eps = test.eps);
            println!(
                "x:{:.eps$}",
                result.decision_variables.transpose(),
                eps = test.eps
            );
        }
    }

    Ok(())
}
