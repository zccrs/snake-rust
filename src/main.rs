extern crate ncursesw;
extern crate ascii;

use ascii::*;
use ncursesw::*;
use ncursesw::normal::*;

fn moveCursor(x: i32, y: i32) {
    r#move(Origin { y: x, x: y }).unwrap();
}

fn setCursorVisiable(visible: bool) {
    curs_set(if visible {CursorType::Visible} else {CursorType::Invisible}).unwrap();
}

fn main() {
    initscr().unwrap();

    if has_colors() {
        start_color();
        use_default_colors();

    } else {
        addstr("terminal has no color support!!!");
    }

    endwin();
}
