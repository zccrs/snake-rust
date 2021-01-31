extern crate ncursesw;

use ncursesw::*;
use ncursesw::normal::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::time::Duration;
use std::thread;

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
    food_y: i16, //记录食物所在地
    colors: ColorPairs
}

struct ColorPairs {
    red: ColorPair,
    yellow: ColorPair,
    cyan: ColorPair,
    white: ColorPair
}

fn move_cursor(x: i32, y: i32) {
    r#move(Origin { y: y, x: x }).expect(&format!("Can't move cursor to ({}, {})", x, y));
}

fn move_cursor_add_char(y: i32, x: i32, c: char) {
    move_cursor(x, y);
    addstr(&c.to_string());
}

fn set_cursor_visiable(visible: bool) {
    curs_set(if visible {CursorType::Visible} else {CursorType::Invisible}).unwrap();
}

fn set_color(color: ColorPair) {
    attrset(Attribute::Bold | color).unwrap();
}

fn addstr(str: &str) {
    // 添加宽字符居然会不显示，醉了
    addwstr(&WideString::from_str(str)).unwrap();
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

    cbreak().unwrap();
    keypad(stdscr(), true).unwrap();
    start_color().unwrap();
    use_default_colors().unwrap();
    noecho().unwrap();

    let global_data = GameData {
        length: 4,
        velocity: 0,
        t1: 0,
        t2: 0,
        t3: 0,
        level: 1,
        HP: 6,
        food: 0,
        food_x: 0,
        food_y: 0,
        colors: ColorPairs {
            red: ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Red), Color::Dark(BaseColor::Black))).unwrap(),
            yellow: ColorPair::new(2, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Black))).unwrap(),
            cyan: ColorPair::new(3, Colors::new(Color::Dark(BaseColor::Cyan), Color::Dark(BaseColor::Black))).unwrap(),
            white: ColorPair::new(4, Colors::new(Color::Dark(BaseColor::White), Color::Dark(BaseColor::Black))).unwrap(),
        }
    };

    start_animation(&global_data);
    getch();

    endwin().unwrap();
}

fn start_animation(data: &GameData) {//绘制启动画面以及隔墙
    set_cursor_visiable(false);//隐藏光标
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut z: i32 = 0;
    clear().unwrap();
    set_color(data.colors.white);//调成白色
    for z in 0..20 {
        if z >= 0 {
            move_cursor(12, z);
            addstr("              ~--______-~                ~-___-~\"       ");
        }
        if z >= 1 {
            move_cursor(12, z - 1);
            addstr("            ~-_           _-~          ~-_       _-~    ");
        }
        if z >= 2 {
            move_cursor(12, z - 2);
            addstr("          \\     ~-____-~     _-~    ~-_    ~-_-~    / ");
        }
        if z >= 3 {
            move_cursor(12, z - 3);
            addstr("         (     (         _-~    _--_    ~-_    _/   |  ");
        }
        if z >= 4 {
            move_cursor(12, z - 4);
            addstr("          /    /            _-~      ~-_        |   |  ");
        }
        if z >= 5 {
            move_cursor(12, z - 5);
            addstr("           /    /              _----_           \\  \\ ");
        }
        if z >= 6 {
            move_cursor(12, z - 6);
            addstr("             /    /                            \\ \\   ");
        }
        if z >= 7 {
            move_cursor(12, z - 7);
            addstr("              /    /                          \\\\     ");
        }
        if z >= 8 {
            move_cursor(12, z - 8);
            addstr("                /    /                      \\\\       ");
        }
        if z >= 9 {
            move_cursor(12, z - 9);
            addstr("                 /     /                   \\            ");
        }
        if z >= 10 {
            move_cursor(12, z - 10);
            addstr("                  |     |                \\                ");
        }
        if z >= 11 {
            move_cursor(12, z - 11);
            addstr("                 \\     \\                                 ");
        }
        if z >= 12 {
            move_cursor(12, z - 12);
            addstr("        \\_______      \\                                  ");
        }
        if z >= 13 {
            move_cursor(12, z - 13);
            addstr(" \\____|__________/  \\                                    ");
        }
        if z >= 14 {
            move_cursor(12, z - 14);
            addstr("\\/     /~     \\_/ \\                                     ");
        }
        if z >= 15 {
            move_cursor(12, z - 15);
            addstr("        _|__|  O|                                          ");
        }
        for k in 15..z {
            move_cursor(12, k - 15);
            addstr("                                                           ");
        }
        refresh().unwrap();
        thread::sleep_ms(20);
    }
    thread::sleep(Duration::from_secs(1));
    clear().unwrap();
    set_color(data.colors.cyan);//调整输出颜色
    j = 60;
    for i in 0..60 { //if是为了异步输出
        if j > 20 {
            move_cursor_add_char(0, 2 * (j - 21), '*'); //输出第一行
        }
        if i < 40 {
            move_cursor_add_char(23, 2 * i, '*'); // 输出最下面一行
        }
        if j > 22 && j < 45 {
            move_cursor_add_char(j - 22, 78, '*'); //输出最右边列
        }
        if i > 22 && j < 45 && j >= 15 {
            move_cursor_add_char(j - 15, 0, '*'); //输出第一列
        }
        if i > 37 && i < 60 {
            move_cursor_add_char(i - 37, 54, '*'); //输出中间那列
            thread::sleep_ms(10);
        }
        j -= 1;
        refresh().unwrap();
        thread::sleep_ms(20);
    }
    move_cursor(56, 11);
    addstr("* * * * * * * * * * * *");                                          //56
    move_cursor(19, 0);
    set_color(data.colors.yellow);//调整输出颜色
    addstr("| | |贪 吃 蛇| | |"); //输出标题
    move_cursor(56, 2);
    addstr("已用时间：");
    move_cursor(75, 2);
    addstr("秒");
    move_cursor(56, 4);
    addstr("生命值：");
    move_cursor(56, 6);
    addstr("当前长度：");
    move_cursor(56, 8);
    addstr("已吃食物：");
    move_cursor(56, 10);
    addstr("第             关");
    move_cursor(64, 12);
    addstr("提示：");
    move_cursor(56, 13);
    addstr("向上：↑   向上：←");
    move_cursor(56, 14);
    addstr("向下：↓   向右：→");
    move_cursor(56, 15);
    addstr("暂停/开始：确定键 ");
    move_cursor(56, 16);
    addstr("重新选关 ：Esc键");
    move_cursor(64, 18);
    addstr("注意！");
    move_cursor(56, 19);
    addstr("1:撞到");
    set_color(data.colors.red);
    addstr("*");
    set_color(data.colors.yellow);//调整输出颜色
    addstr("或墙生命值减一");
    move_cursor(56, 21);
    addstr("2:吃到小星星生命值加一");
    refresh().unwrap();
}