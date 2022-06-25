// Art by Black Sheep

use super::Position;

pub const WORLD_HEIGHT: i32 = 30;
pub const WORLD_MIN_WIDTH: i32 = 80;
pub const WORLD_RIGHT_MARGIN: i32 = 10;
pub const Y_BOTTOM: i32 = WORLD_HEIGHT - 2;

pub const HERO_JUMPING_HEIGHT: i32 = 7;

type STR = &'static str;

pub const DEBUG: STR = "?";


pub const HERO: STR =
r" 0
/#\
/ \";

pub const HERO_CROUCHING_1: STR =
r" o
/#\
| |";

pub const HERO_CROUCHING_2: STR = 
r"/o\
/#\";

pub const HERO_JUMPING_RIGHT: STR =
r" 0
/#‾
/ \";

pub const HERO_JUMPING_LEFT: STR = 
r" 0
‾#\
/ \";

pub const HERO_FALL: STR = 
r"\0/
 #
/ \";


#[cfg(feature = "terminal_backend")]
pub const TUTORIAL: STR = "\
*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
| Quit: Ctrl + Q                          |
* Move left: LEFT ARROW or [              *
| Move right: RIGHT ARROW or ]            |
* Crouch: DOWN ARROW or .                 *
| Jump or Stop crouching: TOP ARROW or /  |
* Cast a spell: any letters + ENTER       *
*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
";

#[cfg(feature = "wasm_backend")]
pub const TUTORIAL: STR = "\
╔═════════════════════════════════════════╗
║ Move: WASD or ARROWS                    ║
║       (W/Up - jump, S/Down - crouch)    ║
║ Fight with your sword: V                ║
║ Cast a spell: SPACE + letters + ENTER   ║
╚═════════════════════════════════════════╝
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


pub const SPIKE_UP: STR = "▲";
pub const SPIKE_DOWN: STR = "▼";


pub const ENEMY: STR = "\
◢█ █◣
◥█ █◤";

pub const GO_RIGHT_SIGN: STR = "\
Go, hero, go! ->";