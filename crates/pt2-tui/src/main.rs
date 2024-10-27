// use std::io;

// use self::app::App;

mod app;
mod tests;

fn main() {
    const EPS: usize = 3;
    const ALPHA_1: f64 = 0.5;
    const ALPHA_2: f64 = 0.9;

    for generate_test in [tests::generate_1] {
        for alpha in [ALPHA_1, ALPHA_2] {
            let test = generate_test();

            let result =
                pt2_core::interior_point(test.c, test.a, test.initial_point, test.b, EPS, ALPHA_1)
                    .unwrap()
                    .last()
                    .unwrap()
                    .unwrap();
        }
    }
}

// fn start_tui() -> io::Result<()> {
//     let mut terminal = ratatui::init();
//     let app_result = App::default().run(&mut terminal);
//     ratatui::restore();
//     app_result
// }
