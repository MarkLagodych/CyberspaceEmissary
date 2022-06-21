pub const HERO: &'static str =
r" o
/#\
/ \";



pub const ROOM_BORDER_HORZ: &'static str = 
"##########################################################################################";

pub const ROOM_BORDER_VERT: &'static str = 
"#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n#\n";

pub const ROOM_WIDTH: i32 = 90;
pub const ROOM_HEIGHT: i32 = 25;



pub const DOOR: &'static str =
"\
.-----.
|     |
|   * |
|     |
|_____|
";

pub const DOOR_WIDTH: usize = 7;
pub const DOOR_HEIGHT: usize = 5;



pub const TUTORIAL: &'static str =
"\
O-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-O
| Quit: Ctrl + Q                          |
* Move left: LEFT ARROW or [              *
| Move right: RIGHT ARROW or ]            |
* Interact: TOP ARROW or DOWN ARROW or '  *
| Choose dialog option: 1-9 + ENTER       |
* Cast a spell: any letters + ENTER       * 
O-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-O
";