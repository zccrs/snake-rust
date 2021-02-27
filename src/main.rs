extern crate ncursesw;

use core::convert::TryFrom;
use ncursesw::shims::bindings::printw;
use std::io::Read;
use ncursesw::*;
use ncursesw::normal::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::time::Duration;
use std::thread;
use std::os::raw::c_int;
use std::os::raw::c_char;
use std::ffi::CString;
use rand::Rng;

struct ColorPairs {
    red: ColorPair,
    yellow: ColorPair,
    cyan: ColorPair,
    white: ColorPair,
    green: ColorPair
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

trait Into_int {
    fn to_int(&self) -> i16;
}

impl TryFrom<&KeyBinding> for Direction {
    type Error = ();

    fn try_from(key: &KeyBinding) -> Result<Direction, ()> {
        match key {
            KeyBinding::UpArrow => Ok(Direction::Up),
            KeyBinding::DownArrow => Ok(Direction::Down),
            KeyBinding::LeftArrow => Ok(Direction::Left),
            KeyBinding::RightArrow => Ok(Direction::Right),
            _ => Err(())
        }
    }
}

impl Into_int for Direction {
    fn to_int(&self) -> i16 {
        match self {
            Direction::Left => 1,
            Direction::Right => 2,
            Direction::Up => 3,
            Direction::Down => 4
        }
    }
}

#[derive(Copy, Clone)]
struct SnakeData {
    x: i16,//蛇身所在横坐标
    y: i16,//蛇身所在纵坐标
    direction: Direction//行走方向
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum ItemType {
    None,
    Food,
    Star,
    Barrier
}

struct GameData {
    length: i16, //n用来记录蛇身长度,初始为4节
    velocity: i32,//用来给记录蛇的移动速度
    t1: i64, //用来记录用时
    t2: i64, //用来记录用时
    t3: i64, //用来记录用时
    level: i8,//用来记录关卡
    hp: i8, //记录蛇的生命值,初始化为6
    food: i16, //用来记录所吃到的食物数
    food_x: i16, //记录食物所在地
    food_y: i16, //记录食物所在地
    colors: ColorPairs,
    snake_infos: [SnakeData; 81],
    map: [[ItemType; 22]; 26]
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
    ncursesw::addstr(str).unwrap();
}

fn getchar() -> u8 {
    let mut buffer = [0; 10];
    std::io::stdin().read(&mut buffer).expect("Failed on getchar");
    buffer[0]
}

fn print_i32(value: i32) {
    unsafe {
        printw(CString::new("%d").unwrap().as_ptr(), value);
    }
}

fn timestamp() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as i64 * 1000i64 + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    ms
}

#[link(name = "c")]
extern "C" {
    fn setlocale(t: c_int, v: *const c_char);
}

fn main() {
    unsafe {
        // 避免使用addstr时中文乱码
        const LC_ALL: i32 = 0;
        setlocale(LC_ALL, CString::new("").unwrap().as_ptr());
    }
    initscr().unwrap();

    if !has_colors() {
        panic!("terminal has no color support!!!");
    }

    cbreak().unwrap();
    keypad(stdscr(), true).unwrap();
    start_color().unwrap();
    use_default_colors().unwrap();
    noecho().unwrap();

    let mut global_data = GameData {
        length: 4,
        velocity: 0,
        t1: 0,
        t2: 0,
        t3: 0,
        level: 1,
        hp: 6,
        food: 0,
        food_x: 0,
        food_y: 0,
        colors: ColorPairs {
            red: ColorPair::new(1, Colors::new(Color::Dark(BaseColor::Red), Color::Dark(BaseColor::Black))).unwrap(),
            yellow: ColorPair::new(2, Colors::new(Color::Dark(BaseColor::Yellow), Color::Dark(BaseColor::Black))).unwrap(),
            cyan: ColorPair::new(3, Colors::new(Color::Dark(BaseColor::Cyan), Color::Dark(BaseColor::Black))).unwrap(),
            white: ColorPair::new(4, Colors::new(Color::Dark(BaseColor::White), Color::Dark(BaseColor::Black))).unwrap(),
            green: ColorPair::new(5, Colors::new(Color::Dark(BaseColor::Green), Color::Dark(BaseColor::Black))).unwrap()
        },
        snake_infos: [SnakeData { x: 0, y: 0, direction: Direction::Up }; 81],
        map: [[ItemType::None; 22]; 26]
    };

    start_animation(&mut global_data);
    loop {
        select_level(&mut global_data);//用来选择关卡并根据关卡设置蛇的移动速度
        set_cursor_visiable(false);//隐藏光标
        if !begin_game(&mut global_data) {
            break;    //游戏结束
        }
    }

    endwin().unwrap();
}

fn start_animation(data: &mut GameData) {//绘制启动画面以及隔墙
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
        thread::sleep(Duration::from_millis(20));
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
            thread::sleep(Duration::from_millis(10));
        }
        j -= 1;
        refresh().unwrap();
        thread::sleep(Duration::from_millis(20));
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

fn select_level(data: &mut GameData)//用来选择关卡并根据关卡设置蛇的移动速度
{
    set_cursor_visiable(true);//显示光标
    data.length = 4; //n用来记录蛇身长度,初始为3节
    data.hp = 6; //记录蛇的生命值,初始化为6
    data.snake_infos[0].x = 6;                  //
    data.snake_infos[0].y = 10;                 //
    data.snake_infos[0].direction = Direction::Right;      //
    data.snake_infos[1].x = 4;                 //
    data.snake_infos[1].y = 10;                //     初始化蛇所在位置和移动方向
    data.snake_infos[1].direction = Direction::Right;     //
    data.snake_infos[2].x = 2;                 //
    data.snake_infos[2].y = 10;                //
    data.snake_infos[2].direction = Direction::Right;    //
    data.snake_infos[3].x = 4; ////////////////
    data.snake_infos[3].y = 4; ///////////////记录蛇尾的信息
    data.snake_infos[3].direction = Direction::Right; ////
    loop {
        move_cursor(15, 3);
        addstr("请输入关数(1-6)：");
        refresh().unwrap();
        data.level = getchar() as i8 - 48;
        if data.level == 0 { //判断是否作弊
            move_cursor(15, 3);
            set_color(data.colors.red);//变成红色
            addstr("  作弊有害智商，需谨慎");
            move_cursor(15, 5);
            set_color(data.colors.yellow);//变成黄色
            addstr("请输入你想要的蛇的生命值：");
            refresh().unwrap();
            data.hp = getchar() as i8 - 48;
            move_cursor(15, 3);
            addstr("                      ");
            move_cursor(15, 5);
            addstr("                                    ");
            continue;//返回选关处
        }
        if data.level < 7 && data.level > 0 {
            break;    //判断关数是否溢出
        }
        move_cursor(15, 5);
        addstr("输入错误！");
        move_cursor(32, 3);
        addstr("          ");
        refresh().unwrap();
    }
    move_cursor(15, 3);
    addstr("                   ");
    match data.level {
        1 => {
            data.velocity = 600;    //
        },
        2 => {
            data.velocity = 400;    //
        },
        3 => {
            data.velocity = 200;    //    根据关数来设定蛇的移动速度
        }
        4 => {
            data.velocity = 150;    //
        }
        5 => {
            data.velocity = 100;    //
        },
        6 => {
            data.velocity = 60;    //
        }
        _ => {
            panic!();
        }
    }
    clear_screen(data);//清除屏幕
}
fn update_data(data: &mut GameData)//用来记录和判断游戏的各种状态数据
{
    move_cursor(66, 2);
    set_color(data.colors.red);//调成红色
    print_i32((data.t1 / 1000) as i32); //程序已用时间
    match data.level {
        1 => {
            move_cursor(59, 10);
            set_color(data.colors.red);//调成红色
            addstr("1");
            set_color(data.colors.yellow);//调成黄色
            addstr(" 2 3 4 5 6");
        },
        2 => {
            move_cursor(59, 10);
            set_color(data.colors.yellow);//调成黄色
            addstr("1 ");
            set_color(data.colors.red);//调成红色
            addstr("2");
            set_color(data.colors.yellow);//调成黄色
            addstr(" 3 4 5 6 ");
        }
        3 => {
            move_cursor(59, 10);
            set_color(data.colors.yellow);//调成黄色
            addstr("1 2 ");
            set_color(data.colors.red);//调成红色
            addstr("3");
            set_color(data.colors.yellow);//调成黄色
            addstr(" 4 5 6 ");
        }
        4 => {
            move_cursor(59, 10);
            set_color(data.colors.yellow);//调成黄色
            addstr("1 2 3 ");
            set_color(data.colors.red);//调成红色
            addstr("4");
            set_color(data.colors.yellow);//调成黄色
            addstr(" 5 6 ");
        }
        5 => {
            move_cursor(59, 10);
            set_color(data.colors.yellow);//调成黄色
            addstr("1 2 3 4 ");
            set_color(data.colors.red);//调成红色
            addstr("5");
            set_color(data.colors.yellow);//调成黄色
            addstr(" 6 ");
        }
        6 => {
            move_cursor(59, 10);
            set_color(data.colors.yellow);//调成黄色
            addstr("1 2 3 4 5 ");
            set_color(data.colors.red);//调成红色
            addstr("6");
        }
        _ => {
            panic!();
        }
    }
    match data.hp {
        1 => {
            move_cursor(65, 4);
            set_color(data.colors.green);//调成绿色
            addstr("▁");
            set_color(data.colors.red);//调成红色
            addstr("▂▃▅▆▇");
        },
        2 => {
            move_cursor(65, 4);
            set_color(data.colors.green);//调成绿色
            addstr("▁▂");
            set_color(data.colors.red);//调成红色
            addstr("▃▅▆▇");
        },
        3 => {
            move_cursor(65, 4);
            set_color(data.colors.green);//调成绿色
            addstr("▁▂▃");
            set_color(data.colors.red);//调成红色
            addstr("▅▆▇");
        },
        4 => {
            move_cursor(65, 4);
            set_color(data.colors.green);//调成绿色
            addstr("▁▂▃▅");
            set_color(data.colors.red);//调成红色
            addstr("▆▇");
        },
        5 => {
            move_cursor(65, 4);
            set_color(data.colors.green);//调成绿色
            addstr("▁▂▃▅▆");
            set_color(data.colors.red);//调成红色
            addstr("▇");
        },
        6 => {
            move_cursor(65, 4);
            set_color(data.colors.green);//调成绿色
            addstr("▁▂▃▅▆▇");
        },
        _ => {
            move_cursor(65, 4);
            set_color(data.colors.green);//调成绿色
            addstr("！超级模式 ！");
        }
    }
    move_cursor(66, 6);
    set_color(data.colors.red);//调成红色
    print_i32(data.length as i32 - 1); //输出蛇的当前长度
    move_cursor(66, 8);
    print_i32(data.food as i32); //输出蛇当前已经吃到食物
    refresh().unwrap();
}
fn clear_screen(data: &mut GameData)//用来清除屏幕
{
    for i in 2..23 {
        move_cursor(2, i);
        addstr("                                                    ");
    }
    data.map[data.food_x as usize][data.food_y as usize]  = ItemType::None;
    refresh().unwrap();
}

fn update_ui(data: &mut GameData)//用来随机产生障碍物以及食物和生命药水以及用来判断游戏的各种参数
{
    let mut a: usize;
    let mut b: usize;
    let mut e: usize;
    let mut f: usize; //a，b用来表示小星星的坐标   c，d代表障碍物坐标
    if data.map[data.food_x as usize][data.food_y as usize] == ItemType::None { //判断食物是不是被吃掉
        loop {
            data.food_x = rand::thread_rng().gen_range(0, 26); //产生随机横坐标
            data.food_y = rand::thread_rng().gen_range(0, 22); //产生随机纵坐标
            if data.map[data.food_x as usize][data.food_y as usize] == ItemType::None {
                break;    //当此处无其他元素是才生效
            }
        }
        data.map[data.food_x as usize][data.food_y as usize] = ItemType::Food; //随机出现食物
        move_cursor((2 * (data.food_x + 1)).into(), (data.food_y + 1).into()); //定位到食物出现的位置
        set_color(data.colors.yellow);//调成黄色
        addstr("●"); //打印出食物
    }
    if data.t1 / 20 > 0 && data.t1 % 12 == 0 && data.t1 > data.t3
        && data.map[(data.snake_infos[0].x as usize - 1) / 2][data.snake_infos[0].y as usize - 1] == ItemType::None {
        loop {
            e = rand::thread_rng().gen_range(0, 26); //产生随机横坐标
            f = rand::thread_rng().gen_range(0, 22); //产生随机纵坐标
            if data.map[e][f] == ItemType::None {
                break;    //当此处无其他元素是才生效
            }
        }
        move_cursor(2 * (e as i32 + 1), f as i32 + 1); //定位到障碍物出现的位置
        data.map[e][f] = ItemType::Barrier; //随机出现障碍物
        set_color(data.colors.red);//调成红色
        addstr("*"); //打印出障碍物
        data.t3 = data.t1; //以免产生多个障碍物
        if data.hp < 7 {
            move_cursor(18, 24);
            set_color(data.colors.white);//调成白色
            addstr("温馨提示：在选关的时候输入0可以开启作弊模式");
        }
    }
    if data.t1 / 25 > 0 && data.t1 % 15 == 0 && data.t1 > data.t3
        && data.map[(data.snake_infos[0].x as usize - 1) / 2][data.snake_infos[0].y as usize - 1] == ItemType::None { //减少星星出现的几率
        loop {
            a = rand::thread_rng().gen_range(0, 26); //产生随机横坐标
            b = rand::thread_rng().gen_range(0, 22); //产生随机纵坐标
            if data.map[a][b] == ItemType::None {
                break;    //当此处无其他元素是才生效
            }
        }
        data.map[a][b] = ItemType::Star; //随机出现小星星（吃到星星长度减1）
        move_cursor(2 * (a as i32 + 1), b as i32 + 1); //定位到星星出现的位置（吃到星星长度减1）
        set_color(data.colors.green);//调成绿色
        addstr("☆"); //打印出星星（吃到星星长度减1）
        data.t3 = data.t1; //以免产生多个障碍物
        if data.hp < 7 {
            move_cursor(18, 24);
            addstr("                                            ");
        }
    }
    for i in 0..data.length as usize {
        if data.map[(data.snake_infos[i].x as usize - 1) / 2][data.snake_infos[i].y as usize - 1] == ItemType::Food { //判断蛇是否吃到食物
            data.length += 1;//让蛇长度加1
            data.food += 1;//将食物数加1
            data.map[(data.snake_infos[i].x as usize - 1) / 2][data.snake_infos[i].y as usize - 1] = ItemType::None; //让食物标示归零
            break;
        }
    }
    if data.map[(data.snake_infos[0].x as usize - 1) / 2][data.snake_infos[0].y as usize - 1] == ItemType::Star { //判断蛇是否吃到星星
        data.map[(data.snake_infos[0].x as usize - 1) / 2][data.snake_infos[0].y as usize - 1] = ItemType::None; //让星星标示归零
        if data.hp < 6 {
            data.hp += 1;    //将生命值加1
        }
    }
    data.t1 = timestamp() - data.t2; //刷新游戏运行时间
    refresh().unwrap();
}

fn get_key() -> Result<KeyBinding, ()>
{
    match getch() {
        Ok(result) => {
            match result {
                CharacterResult::Key(key) => {
                    return Ok(key)
                },
                CharacterResult::Character(ch) => {
                    if ch == '\n' {
                        return Ok(KeyBinding::Enter)
                    } else if ch == '\u{001B}' { // Esc key
                        return Ok(KeyBinding::Exit)
                    } else {
                        return Err(())
                    }
                }
            }
        },
        Err(_) => return Err(())
    };
}

fn handle_key_event(data: &mut GameData)//用户是否操作键盘
{
    halfdelay(Duration::from_secs(10)).expect("halfdelay failed!");
    let key = match get_key() {
        Ok(result) => {
            result
        },
        Err(_) => return
    };
    match Direction::try_from(&key) {
        Ok(direction) => {
            if (direction.to_int() + data.snake_infos[0].direction.to_int() != Direction::Up.to_int() + Direction::Down.to_int())
                && (direction.to_int() + data.snake_infos[0].direction.to_int() != Direction::Left.to_int() + Direction::Right.to_int())
                && direction != data.snake_infos[0].direction { //判断按键是否是方向键，并且是不是蛇移动方向的反方向
                data.snake_infos[0].direction = direction;    //如果不是就改变蛇头方向
            }
            return;
        },
        Err(_) => {}
    };

    if key == KeyBinding::Enter { //判断用户是否暂停
        let a: i64;
        let b: i64;
        a = timestamp(); //记录当前程序已用时间
        move_cursor(20, 1);
        set_color(data.colors.white);//调成白色
        addstr("已暂停,按确定键开始");
        loop {
            match get_key() {
                Ok(key) => {
                    if key == KeyBinding::Enter {////判断是否按键且是否解除暂停
                        move_cursor(20, 1);
                        addstr("                     "); //清除"已暂停,按确定键开始"这行字
                        break;
                    }
                },
                Err(_) => continue
            }
        }
        b = timestamp(); //记录当前程序已用时间
        data.t2 += b - a; //将暂停加到t2上供t1减去
    } else if key == KeyBinding::Exit { //判断是否重新选关
        select_level(data);//用来选择关卡并根据关卡设置蛇的移动速度
        begin_game(data);//开始游戏
    }
}
fn begin_game(data: &mut GameData) -> bool
{
    set_cursor_visiable(false);
    let mut n: i16 = 0;
    //int ch = RIGHT; //向右
    data.t2 = timestamp(); //记录当前程序已用时间
    loop {
        data.t1 = timestamp() - data.t2; //刷新游戏运行时间
        update_data(data);//用来记录游戏的各种状态数据
        move_cursor(data.snake_infos[0].x.into(), data.snake_infos[0].y.into()); //转到蛇头位置
        set_color(data.colors.red);//改成红色
        addstr("◆"); //打印蛇头
        for i in 1..data.length - 1 {
            move_cursor(data.snake_infos[i as usize].x.into(), data.snake_infos[i as usize].y.into()); //转到当前蛇身位置
            set_color(data.colors.yellow);//改成黄色
            addstr("●"); //打印蛇身
        }
        move_cursor(data.snake_infos[data.length as usize - 2].x.into(), data.snake_infos[data.length as usize - 2].y.into()); //转到当前蛇尾位置
        set_color(data.colors.red);//改成红色
        addstr("●"); //打印蛇尾
        thread::sleep(Duration::from_millis(data.velocity as u64));//控制蛇的移动速度
        data.t1 = timestamp() - data.t2; //刷新游戏运行时间
        move_cursor(data.snake_infos[data.length as usize - 1].x.into(), data.snake_infos[data.length as usize - 1].y.into()); //移到蛇尾所在地
        addstr(" "); //清除上个循环的蛇尾
        for i in (1..data.length as usize).rev() {
            data.snake_infos[i] = data.snake_infos[i - 1];    //移动蛇
        }
        handle_key_event(data);//用户是否操作键盘
        match data.snake_infos[0].direction {
            Direction::Up => {
                data.snake_infos[0].y -= 1;    //改变蛇头坐标，移动蛇头
            }
            Direction::Down => {
                data.snake_infos[0].y += 1;    //改变蛇头坐标，移动蛇头
            }
            Direction::Left => {
                data.snake_infos[0].x -= 2;    //改变蛇头坐标，移动蛇头
            }
            Direction::Right => {
                data.snake_infos[0].x += 2;    //改变蛇头坐标，移动蛇头
            }
        }
        if data.snake_infos[0].x == 0 { //当蛇撞到左墙时
            data.hp -= 1;//将生命值减一
            data.snake_infos[0].x = 52; //将其穿墙
        }
        if data.snake_infos[0].x == 54 { //当蛇撞到右墙时
            data.hp -= 1;//将生命值减一
            data.snake_infos[0].x = 2; //将其穿墙
        }
        if data.snake_infos[0].y == 0 { //当蛇撞到上墙时
            data.hp -= 1;//将生命值减一
            data.snake_infos[0].y = 22; //将其穿墙
        }
        if data.snake_infos[0].y == 23 { //当蛇撞到下墙时
            data.hp -= 1;//将生命值减一
            data.snake_infos[0].y = 1; //将其穿墙
        }
        for i in 1..data.length as usize - 1 {
            if data.snake_infos[0].x == data.snake_infos[i].x && data.snake_infos[0].y == data.snake_infos[i].y {
                n = data.length + 1;    //判断蛇是否撞到自
            }
        }
        if n >= data.length { //当蛇撞到自己
            data.hp = 0; //将蛇死亡
        }
        if data.map[(data.snake_infos[0].x - 1) as usize / 2][data.snake_infos[0].y as usize - 1] == ItemType::Barrier { //当蛇障碍物时
            data.hp -= 1;//将生命值减一
            data.map[(data.snake_infos[0].x - 1) as usize / 2][data.snake_infos[0].y as usize - 1] = ItemType::None;
        }
        if data.hp == 0 {
            move_cursor(25, 5);
            set_color(data.colors.white);//调成白色
            addstr("游戏结束！！！");
            thread::sleep(Duration::from_secs(3));//延时
            return true;
        }
        if data.length == 81 {
            move_cursor(25, 5);
            set_color(data.colors.white);//调成白色
            addstr("恭喜你过关！！！");
            thread::sleep(Duration::from_secs(3));//延时
            return true;
        }
        update_ui(data);//用来随机产生障碍物以及食物和生命药水以及用来判断游戏的各种参数（小星星是否吃到，是否撞墙)
    }
}