use std::io;
use std::io::Write;
use std::str::FromStr;
use strum_macros::EnumString;

mod board;
pub use board::{Board};

fn main() {
    let board = Board::new();
}
