extern crate ncursesw;

use ncursesw::*;

fn moveCursor(x: i32, y: i32) {
    r#move(Origin { y: x, x: y });
}

fn main() {

}
