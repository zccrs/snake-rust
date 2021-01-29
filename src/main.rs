extern crate ncursesw;

use ncursesw::*;
use ncursesw::normal::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn moveCursor(x: i32, y: i32) {
    r#move(Origin { y: x, x: y }).unwrap();
}

fn setCursorVisiable(visible: bool) {
    curs_set(if visible {CursorType::Visible} else {CursorType::Invisible}).unwrap();
}

fn setColor(color: BaseColor) {
    let color = Colors::new(Color::Dark(color), Color::Dark(BaseColor::Black));
    attrset(Attribute::Dim | ColorPair::new(1, color).unwrap());
}

fn timestamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    ms
}

fn main() {
    initscr().unwrap();

    if !has_colors() {
        panic!("terminal has no color support!!!");
    }

    start_color();
    use_default_colors();
    noecho().unwrap();

    endwin();
}
