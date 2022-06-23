// Art by Black Sheep

pub const WORLD_HEIGHT: i32 = 25;
pub const WORLD_MIN_WIDTH: i32 = 80;
pub const WORLD_RIGHT_MARGIN: i32 = 10;


type STR = &'static str;

pub const DEBUG: STR = "?";


pub const HERO: STR =
r" 0
/#\
/ \";


pub const TUTORIAL: STR = "\
*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
| Quit: Ctrl + Q                          |
* Move left: LEFT ARROW or [              *
| Move right: RIGHT ARROW or ]            |
* Jump: TOP ARROW or '                    *
| Cast a spell: any letters + ENTER       |
*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
";


pub const FLOOR: [char; 300] = ['-'; 300];


pub const SWORD_1: STR = "

▛";
pub const SWORD_2: STR = "
■■
 ▔";
pub const SWORD_3: STR = "
■■■";
pub const SWORD_4: STR = "\
 ▁
■■";
pub const SWORD_5: STR = "\
▙";