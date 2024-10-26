use std::io;

use self::app::App;

mod app;

fn main() {
    pt2_core::interior_point();
}

fn start_tui() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
