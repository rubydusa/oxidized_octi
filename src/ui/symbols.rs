use super::super::board::{Arrow, Octi};

pub const HORIZONTAL_LINE: &'static str = "─";
pub const VERTICAL_LINE: &'static str = "│";

pub const RIGHT_UP_EDGE: &'static str = "└";
pub const RIGHT_DOWN_EDGE: &'static str = "┌";
pub const LEFT_UP_EDGE: &'static str = "┘";
pub const LEFT_DOWN_EDGE: &'static str = "┐";

pub const RIGHT_BORDER: &'static str = "┤";
pub const LEFT_BORDER: &'static str = "├";
pub const UP_BORDER: &'static str = "┬";
pub const DOWN_BORDER: &'static str = "┴";

pub const INTERSECTION: &'static str = "┼";

pub const OCTI_TOP: &'static str = "╱▔▔╲";
pub const OCTI_BOTTOM: &'static str = "╲▁▁╱";
pub const OCTI_RIGHT_SIDE: &'static str = "▕";
pub const OCTI_LEFT_SIDE: &'static str = "▏";

pub const RED_OCTI_ARROW: &'static str = "╱╲";
pub const GREEN_OCTI_ARROW: &'static str = "╲╱";

const HORIZONTAL_ARROW: &'static str = "──";
const VERTICAL_LINE_ARROW: &'static str = "▕▏";
const RIGHT_SLANTED_ARROW: &'static str = "╱";
const LEFT_SLANTED_ARROW: &'static str = "╲";

const NO_HORIZONTAL_ARROW: &'static str = "  ";
const NO_VERTICAL_LINE_ARROW: &'static str = "  ";
const NO_RIGHT_SLANTED_ARROW: &'static str = " ";
const NO_LEFT_SLANTED_ARROW: &'static str = " ";

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
