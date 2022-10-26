use std::io;
use std::io::Write;
use std::str::FromStr;
use strum_macros::EnumString;

mod board;
pub use board::{Board, Team, Pieces};

fn main() {
    let test = Board::new();
    let test2 = test.position.get(&Team::White).unwrap().get(&Pieces::Pawn).unwrap();

}
