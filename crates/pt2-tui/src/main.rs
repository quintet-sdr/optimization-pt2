// use std::io;

// use self::app::App;

mod app;

fn main() {
    const EPS: usize = 3;

    let iterations = pt2_core::interior_point(
        vec![9., 10., 16.],
        vec![vec![18., 15., 12.], vec![6., 4., 8.], vec![5., 3., 3.]],
        vec![1., 1., 1., 315., 174., 169.],
        &[360., 192., 180.],
        0.5,
        EPS,
    )
    .unwrap();

    for (i, iteration) in iterations.enumerate().map(|(i, it)| (i + 1, it)) {
        let iteration = iteration.unwrap();
        println!("Iteration {i}");

        println!("max: {:.EPS$}", iteration.max);
        println!("x:{:.EPS$}", iteration.x);
    }
}

// fn start_tui() -> io::Result<()> {
//     let mut terminal = ratatui::init();
//     let app_result = App::default().run(&mut terminal);
//     ratatui::restore();
//     app_result
// }
