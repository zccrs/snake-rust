extern crate ncursesw;

use ncursesw::*;
use ncursesw::normal::*;
use std::time::{SystemTime, UNIX_EPOCH};

struct GameData {
    length: i16, //n用来记录蛇身长度,初始为4节
    velocity: i16,//用来给记录蛇的移动速度
    t1: i32, //用来记录用时
    t2: i32, //用来记录用时
    t3: i32, //用来记录用时
    level: i8,//用来记录关卡
    HP: i8, //记录蛇的生命值,初始化为6
    food: i16, //用来记录所吃到的食物数
    food_x: i16, //记录食物所在地
    food_y: i16 //记录食物所在地
}

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
