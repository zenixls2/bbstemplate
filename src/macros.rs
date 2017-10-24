#![allow(unused_macros)]
#![allow(non_upper_case_globals)]

pub const BACKSPACE: u8 = 8;

macro_rules! rgb2ansi {
    ($r:tt, $b:tt, $g:tt) => {
        (16 + 36 * $r + 6 * $g + $b)
    };
}

macro_rules! u2a {
    ($u:tt, $a:tt) => {
        if $u >= 100 {
            $a.push((($u / 100) % 10 + 48) as u8);
            $a.push((($u / 10) % 10 + 48) as u8);
            $a.push(($u % 10 + 48) as u8);
        } else if $u >= 10 {
            $a.push((($u / 10) % 10 + 48) as u8);
            $a.push(($u % 10 + 48) as u8);
        } else {
            $a.push(($u + 48) as u8);
        }
    }
}

macro_rules! gen_command {
    ($name:ident, $arg:ident, $cap:tt, $code:tt) => {
        pub fn $name($arg: u16) -> Box<[u8]> {
            let mut out: Vec<u8> = Vec::with_capacity($cap);
            out.push(27);
            out.push(91);
            u2a!($arg, out);
            out.push($code);
            return out.into_boxed_slice();
        }
    };
    ($name:ident, $arg1:ident, $arg2:ident, $cap:tt, $code:tt) => {
        pub fn $name($arg1: u16, $arg2: u16) -> Box<[u8]> {
            let mut out: Vec<u8> = Vec::with_capacity($cap);
            out.push(27);
            out.push(91);
            u2a!($arg1, out);
            out.push(59);
            u2a!($arg2, out);
            out.push($code);
            return out.into_boxed_slice();
        }
    };
}

pub const MOUSE_INIT_UTF8: [u8; 8] = [
    // \x1b[?1005h
    27, 91, 63, 49, 48, 48, 53, 104
];

pub const MOUSE_INIT_URXVT: [u8; 8] = [
    // \x1b[?1015h
    27, 91, 63, 49, 48, 49, 53, 104
];

pub const MOUSE_INIT_XTERM: [u8; 8] = [
    // \x1b[?1006h
    27, 91, 63, 49, 48, 48, 54, 104
];

pub const MOUSE_INIT_MOVEMENT: [u8; 8] = [
    // \x1b[?1003h, enable movement, drag, drop reporting
    27, 91, 63, 49, 48, 48, 51, 104
];

pub const MOUSE_END_MOVEMENT: [u8; 8] = [
    // \x1b[?1003l, disable mouse movement events
    27, 91, 63, 49, 48, 48, 51, 108
];

pub fn trueColorFg(r: u8, g: u8, b: u8) -> Box<[u8]> {
    // \x1b[38;2;{};{};{}m
    let mut out = Vec::with_capacity(19);
    out.push(27);
    out.push(91);
    out.push(51);
    out.push(56);
    out.push(59);
    // true color RGB
    out.push(50);
    out.push(59);
    u2a!(r, out);
    out.push(59);
    u2a!(g, out);
    out.push(59);
    u2a!(b, out);
    out.push(109);
    return out.into_boxed_slice();
}

pub fn ansiColorFg(c: u8) -> Box<[u8]> {
    // ansi fg color \x1b[38;5;{}m
    let mut out = Vec::with_capacity(11);
    out.push(27);
    out.push(91);
    out.push(51);
    out.push(56);
    out.push(59);
    // ansi color
    out.push(53);
    out.push(59);
    u2a!(c, out);
    out.push(109);
    return out.into_boxed_slice();
}

macro_rules! colorFg {
    ($r:tt, $g:tt, $b:tt) => {
        trueColorFg($r, $g, $b)
    };
    ($c:tt) => {
        ansiColorFg($c)
    };
    () => {
        [27, 91, 51, 56, 109]
    };
}

pub fn trueColorBg(r: u8, g: u8, b: u8) -> Box<[u8]> {
    // \x1b[48;2;{};{};{}m
    let mut out = Vec::with_capacity(19);
    out.push(27);
    out.push(91);
    out.push(52);
    out.push(56);
    out.push(59);
    // true color RGB
    out.push(50);
    out.push(59);
    u2a!(r, out);
    out.push(59);
    u2a!(g, out);
    out.push(59);
    u2a!(b, out);
    out.push(109);
    return out.into_boxed_slice();
}

pub fn ansiColorBg(c: u8) -> Box<[u8]> {
    // ansi bg color \x1b[48;5;{}m
    let mut out = Vec::with_capacity(11);
    out.push(27);
    out.push(91);
    out.push(52);
    out.push(56);
    out.push(59);
    // ansi color
    out.push(53);
    out.push(59);
    u2a!(c, out);
    out.push(109);
    return out.into_boxed_slice();
}

macro_rules! colorBg {
    ($r:tt, $g:tt, $b:tt) => {
        trueColorBg($r, $g, $b)
    };
    ($c:tt) => {
        ansiColorBg($c)
    };
    () => {
        [27, 91, 52, 56, 109]
    }
}

pub fn imgcat(img: &[u8], width: u8, height: u8) -> Box<[u8]> {
    let length = img.len();
    let mut out: Vec<u8> = Vec::with_capacity(length + 43);
    out.push(27);
    out.push(93);
    out.push(49);
    out.push(51);
    out.push(51);
    out.push(55);
    out.push(59);
    out.push(70);
    out.push(105);
    out.push(108);
    out.push(101);
    out.push(61);
    out.push(105);
    out.push(110);
    out.push(108);
    out.push(105);
    out.push(110);
    out.push(101);
    out.push(61);
    out.push(49);
    out.push(59);
    out.push(119);
    out.push(105);
    out.push(100);
    out.push(116);
    out.push(104);
    out.push(61);
    u2a!(width, out);
    out.push(59);
    out.push(104);
    out.push(101);
    out.push(105);
    out.push(103);
    out.push(104);
    out.push(116);
    out.push(61);
    u2a!(height, out);
    out.push(58);
    out.extend_from_slice(img);
    out.push(7);
    return out.into_boxed_slice();
}


macro_rules! reverseVideo {
    // \x1b[?5h
    () => {[27, 91, 63, 53, 104]};
}


macro_rules! normalVideo {
    // \x1b[?5l
    () => {[27, 91, 63, 53, 108]};
}


macro_rules! normal {
    // \x1b[m
    () => {[27, 91, 109]};
}


macro_rules! bold {
    // \x1b[1m
    () => {[27, 91, 49, 109]};
}


macro_rules! underline {
    // \x1b[4m
    () => {[27, 91, 52, 109]};
}


macro_rules! reverse {
    // \x1b[7m
    () => {[27, 91, 55, 109]};
}


macro_rules! black {
    // \x1b[30m
    () => {[27, 91, 51, 48, 109]};
}


macro_rules! red {
    // \x1b[31m
    () => {[27, 91, 51, 49, 109]};
}


macro_rules! green {
    // \x1b[32m
    () => {[27, 91, 51, 50, 109]};
}


macro_rules! yellow {
    // \x1b[33m
    () => {[27, 91, 51, 51, 109]};
}


macro_rules! purple {
    // \x1b[34m;
    () => {[27, 91, 51, 52, 109]};
}


macro_rules! pink {
    // \x1b[35m
    () => {[27, 91, 51, 53, 109]};
}


macro_rules! cyan {
    // \x1b[36m
    () => {[27, 91, 51, 54, 109]};
}


macro_rules! white {
    // \x1b[37m
    () => {[27, 91, 51, 55, 109]};
}


macro_rules! backBlack {
    // \x1b[40m
    () => {[27, 91, 52, 48, 109]};
}


macro_rules! backRed {
    // \x1b[41m
    () => {[27, 91, 52, 49, 109]};
}


macro_rules! backGreen {
    // \x1b[42m
    () => {[27, 91, 52, 50, 109]};
}



macro_rules! backYellow {
    // \x1b[43m
    () => {[27, 91, 52, 51, 109]};
}


macro_rules! backBlue {
    // \x1b[44m
    () => {[27, 91, 52, 52, 109]};
}


macro_rules! backPurple {
    // \x1b[45m
    () => {[27, 91, 52, 53, 109]};
}


macro_rules! backLightBlue {
    // \x1b[46m
    () => {[27, 91, 52, 54, 109]};
}


macro_rules! backWhite {
    // \x1b[47m
    () => {[27, 91, 52, 55, 109]};
}


macro_rules! backLightRed {
    // \x1b[48m
    () => {[27, 91, 52, 56, 109]};
}

gen_command!(_move, line, column, 9, 72);

macro_rules! move2 {
    ($line:tt, $column:tt) => {_move($line, $column)};
}


macro_rules! clear {
    // \x1b[2J
    () => {[27, 91, 50, 74]};
}

gen_command!(_up, line, 5, 65);


macro_rules! up {
    ($line:tt) => {_up($line)};
}

gen_command!(_down, line, 5, 66);


macro_rules! down {
    ($line:tt) => {_down($line)};
}

gen_command!(_right, column, 6, 67);


macro_rules! right {
    ($column:tt) => {_right($column)};
}

gen_command!(_left, column, 6, 68);


macro_rules! left {
    ($column:tt) => {_left($column)};
}

macro_rules! home {
    // \x1b[H
    () => {[27, 91, 72]};
}

macro_rules! scrollUp {
    // \x1bD
    () => {[27, 68]};
}

macro_rules! scrollDown {
    // \x1bM
    () => {[27, 77]};
}

macro_rules! nextLine {
    // \x1bE
    () => {[27, 69]};
}


macro_rules! save {
    // \x1b7
    () => {[27, 55]};
}


macro_rules! restore {
    // \x1b8
    () => {[27, 56]};
}


macro_rules! bigTop {
    // Double height letters, top half
    // \x1b#3
    () => {[27, 35, 51]};
}


macro_rules! bigBottom {
    // Double height letters, bottom half
    // \x1b#4
    () => {[27, 35, 52]};
}


macro_rules! singleWidth {
    // single width, single height letters
    // \x1b#5
    () => {[27, 35, 53]};
}


macro_rules! doubleWidth {
    // double width, single height letters
    // \x1b#6
    () => {[27, 35, 54]};
}


macro_rules! clearRight {
    // clear line from cursor right
    // \x1b[K
    () => {[27, 91, 75]};
}


macro_rules! clearLeft {
    // clear line from cursor left
    // \x1b[1K
    () => {[27, 91, 49, 75]};
}


macro_rules! clearLine {
    // clear entire line
    // \x1b[2K
    () => {[27, 91, 50, 75]};
}


macro_rules! clearDown {
    // clear screen from cursor down
    // \x1b[J
    () => {[27, 91, 74]};
}


macro_rules! clearUp {
    // clear screen from cursor up
    // \x1b[1J
    () => {[27, 91, 49, 74]};
}


macro_rules! reset {
    // reset terminal to initial state
    // \x1bc
    () => {[27, 99]};
}


macro_rules! blinking {
    // turn blinking mode on
    // \x1b[5m
    () => {[27, 91, 53, 109]};
}


macro_rules! lowIntensity {
    // turn lowIntensity mode on
    // \x1b[2m
    () => {[27, 91, 50, 109]};
}


macro_rules! clearTab {
    // clear a tab at current column
    // \x1b[g
    () => {[27, 91, 103]};
}


macro_rules! cursorUp {
    // Move cursor up one line
    // \x1b[A
    () => {[27, 91, 49, 65]};
}


macro_rules! cursorDown {
    // Move cursor down one line
    // \x1b[1B
    () => {[27, 91, 49, 66]};
}


macro_rules! cursorRight {
    // Move cursor right one char
    // \x1b[1C
    () => {[27, 91, 49, 67]};
}


macro_rules! cursorLeft {
    // Move cursor left one char
    // \x1b[1D
    () => {[27, 91, 49, 68]};
}


macro_rules! cursorHome {
    // Move cursor to upper left corner
    // \x1bH
    () => {[27, 72]};
}


macro_rules! revIndex {
    // Generate a reverse line-feed
    // \x1bI
    () => {[27, 73]};
}


macro_rules! clearEOS {
    // Clear to end of screen
    // \x1bJ
    () => {[27, 74]};
}


macro_rules! clearEOL {
    // Erase to end of current line
    // \x1bK
    () => {[27, 75]};
}

macro_rules! TBC2  {
    // Clear a tab at current column
    // \x1b[0g
    () => {[27, 91, 48, 103]};
}

macro_rules! TBC3 {
    // Clear all tabs
    // \x1b[3g
    () => {[27, 91, 51, 103]};
}

macro_rules! invisible {
    // Turn invisible text mode on
    // \x1b[8m
    () => {[27, 91, 56, 109]};
}

macro_rules! tab {
    // set a tab at the current column
    // \x1bH
    () => {[27, 72]};
}
