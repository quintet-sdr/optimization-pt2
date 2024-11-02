mod tests;

fn main() {
    const EPS: usize = 2;
    const ALPHA_1: f64 = 0.5;
    const ALPHA_2: f64 = 0.9;

    for generate_test in tests::generators() {
        for alpha in [ALPHA_1, ALPHA_2] {
            let test = generate_test();

            let iterations = match pt2_core::interior_point(
                test.objective_function,
                test.constraints,
                test.initial_point,
                EPS,
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

            println!("alpha: {alpha:.EPS$}");
            println!("max: {:.EPS$}", result.max);
            println!("x:{:.EPS$}", result.decision_variables.transpose());
        }
    }
}
