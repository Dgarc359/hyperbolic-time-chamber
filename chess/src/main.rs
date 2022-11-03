use std::io;
use std::io::Write;
use std::str::FromStr;
use strum_macros::EnumString;

mod board;
pub use board::{Board};

fn main() {
    let mut board = Board::new();
    board.move_piece(0, 1);
    let new_piece = board.position.get(&1).unwrap().to_owned();
    println!("{:?}", new_piece);
}
