// use std::io;

// use self::app::App;

mod app;

fn main() {
    const EPS: usize = 2;

    let _ = pt2_core::interior_point(
        vec![9., 10., 16.],
        vec![vec![18., 15., 12.], vec![6., 4., 8.], vec![5., 3., 3.]],
        vec![1., 1., 1., 315., 174., 169.],
        vec![360., 192., 180.],
        0.5,
        EPS,
    );
}

// fn start_tui() -> io::Result<()> {
//     let mut terminal = ratatui::init();
//     let app_result = App::default().run(&mut terminal);
//     ratatui::restore();
//     app_result
// }
