// ansi.rs
// Stores ANSI color code data.
// © 2025 Shuntaro Kasatani

// Control sequences
#[allow(unused)]
pub const RESET: &str = "\x1b[0m";
#[allow(unused)]
pub const BOLD: &str = "\x1b[1m";
#[allow(unused)]
pub const DIM: &str = "\x1b[2m";
#[allow(unused)]
pub const ITALIC: &str = "\x1b[3m";
#[allow(unused)]
pub const UNDERLINE: &str = "\x1b[4m";
#[allow(unused)]
pub const BLINK: &str = "\x1b[5m";
#[allow(unused)]
pub const REVERSE: &str = "\x1b[7m";
#[allow(unused)]
pub const HIDDEN: &str = "\x1b[8m";
#[allow(unused)]
pub const STRIKETHROUGH: &str = "\x1b[9m";

// Foreground (text) colors
#[allow(unused)]
pub const BLACK: &str = "\x1b[30m";
#[allow(unused)]
pub const RED: &str = "\x1b[31m";
#[allow(unused)]
pub const GREEN: &str = "\x1b[32m";
#[allow(unused)]
pub const YELLOW: &str = "\x1b[33m";
#[allow(unused)]
pub const BLUE: &str = "\x1b[34m";
#[allow(unused)]
pub const MAGENTA: &str = "\x1b[35m";
#[allow(unused)]
pub const CYAN: &str = "\x1b[36m";
#[allow(unused)]
pub const WHITE: &str = "\x1b[37m";

// Bright foreground colors
#[allow(unused)]
pub const BRIGHT_BLACK: &str = "\x1b[90m"; // Usually renders as dark gray
#[allow(unused)]
pub const BRIGHT_RED: &str = "\x1b[91m";
#[allow(unused)]
pub const BRIGHT_GREEN: &str = "\x1b[92m";
#[allow(unused)]
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
#[allow(unused)]
pub const BRIGHT_BLUE: &str = "\x1b[94m";
#[allow(unused)]
pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
#[allow(unused)]
pub const BRIGHT_CYAN: &str = "\x1b[96m";
#[allow(unused)]
pub const BRIGHT_WHITE: &str = "\x1b[97m";

// Background colors
#[allow(unused)]
pub const BG_BLACK: &str = "\x1b[40m";
#[allow(unused)]
pub const BG_RED: &str = "\x1b[41m";
#[allow(unused)]
pub const BG_GREEN: &str = "\x1b[42m";
#[allow(unused)]
pub const BG_YELLOW: &str = "\x1b[43m";
#[allow(unused)]
pub const BG_BLUE: &str = "\x1b[44m";
#[allow(unused)]
pub const BG_MAGENTA: &str = "\x1b[45m";
#[allow(unused)]
pub const BG_CYAN: &str = "\x1b[46m";
#[allow(unused)]
pub const BG_WHITE: &str = "\x1b[47m";

// Bright background colors
#[allow(unused)]
pub const BG_BRIGHT_BLACK: &str = "\x1b[100m";
#[allow(unused)]
pub const BG_BRIGHT_RED: &str = "\x1b[101m";
#[allow(unused)]
pub const BG_BRIGHT_GREEN: &str = "\x1b[102m";
#[allow(unused)]
pub const BG_BRIGHT_YELLOW: &str = "\x1b[103m";
#[allow(unused)]
pub const BG_BRIGHT_BLUE: &str = "\x1b[104m";
#[allow(unused)]
pub const BG_BRIGHT_MAGENTA: &str = "\x1b[105m";
#[allow(unused)]
pub const BG_BRIGHT_CYAN: &str = "\x1b[106m";
#[allow(unused)]
pub const BG_BRIGHT_WHITE: &str = "\x1b[107m";
