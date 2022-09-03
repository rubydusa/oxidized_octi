use super::super::board::{Arrow, Octi};

pub const HORIZONTAL_LINE: &str = "─";
pub const VERTICAL_LINE: &str = "│";

pub const RIGHT_UP_EDGE: &str = "└";
pub const RIGHT_DOWN_EDGE: &str = "┌";
pub const LEFT_UP_EDGE: &str = "┘";
pub const LEFT_DOWN_EDGE: &str = "┐";

pub const RIGHT_BORDER: &str = "┤";
pub const LEFT_BORDER: &str = "├";
pub const UP_BORDER: &str = "┬";
pub const DOWN_BORDER: &str = "┴";

pub const INTERSECTION: &str = "┼";

pub const OCTI_TOP: &str = "╱▔▔╲";
pub const OCTI_BOTTOM: &str = "╲▁▁╱";
pub const OCTI_RIGHT_SIDE: &str = "▕";
pub const OCTI_LEFT_SIDE: &str = "▏";

pub const RED_OCTI_ARROW: &str = "╱╲";
pub const GREEN_OCTI_ARROW: &str = "╲╱";

const HORIZONTAL_ARROW: &str = "──";
const VERTICAL_LINE_ARROW: &str = "▕▏";
const RIGHT_SLANTED_ARROW: &str = "╱";
const LEFT_SLANTED_ARROW: &str = "╲";

const NO_HORIZONTAL_ARROW: &str = "  ";
const NO_VERTICAL_LINE_ARROW: &str = "  ";
const NO_RIGHT_SLANTED_ARROW: &str = " ";
const NO_LEFT_SLANTED_ARROW: &str = " ";

pub fn arrow_symbol(octi: &Octi, arr: usize) -> &'static str {
    if octi.has_arr(&Arrow::new(arr).unwrap()) {
        match arr {
            0 | 4 => HORIZONTAL_ARROW,
            1 | 5 => RIGHT_SLANTED_ARROW,
            2 | 6 => VERTICAL_LINE_ARROW,
            3 | 7 => LEFT_SLANTED_ARROW,
            _ => panic!("Arrow unwrap did not fail"),
        }
    } else {
        match arr {
            0 | 4 => NO_HORIZONTAL_ARROW,
            1 | 5 => NO_RIGHT_SLANTED_ARROW,
            2 | 6 => NO_VERTICAL_LINE_ARROW,
            3 | 7 => NO_LEFT_SLANTED_ARROW,
            _ => panic!("Arrow unwrap did not fail"),
        }
    }
}
