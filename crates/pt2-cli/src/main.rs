use crate::tests::Lpp;

mod tests;

fn main() {
    const EPS: usize = 3;
    const ALPHA_1: f64 = 0.5;
    const ALPHA_2: f64 = 0.9;

    for generate_test in [tests::generate_1] {
        for alpha in [ALPHA_1, ALPHA_2] {
            let Lpp {
                objective_function: c,
                constraints: a,
                initial_point,
                rhs_numbers: b,
            } = generate_test();

            let result = pt2_core::interior_point(c, a, initial_point, b, EPS, alpha)
                .unwrap()
                .last()
                .unwrap()
                .unwrap();

            println!("alpha: {alpha:.EPS$}");
            println!("max: {:.EPS$}", result.max);
            println!("x:{:.EPS$}", result.x.transpose());
        }
    }
}
