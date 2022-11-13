use std::io;
use std::io::Write;
use std::str::FromStr;
use board::BoardPos;
use strum_macros::EnumString;

mod board;
pub use board::{Board, Team, Pieces};

fn main() {
    let mut test = Board::new();
    println!("Test1: {:?}", test.position.get(&BoardPos{x: 1, y: 1}).unwrap());
    println!("Test2: {:?}", test.position.get(&BoardPos { x: 1, y: 2 }));
    test.move_piece(BoardPos {x: 1, y: 1}, BoardPos { x: 1, y: 2 });

    println!("Test3: {:?}", test.position.get(&BoardPos { x: 1, y: 2 }).unwrap());

    test.move_piece(BoardPos {x: 1, y: 2}, BoardPos { x: 1, y: 6 });
    println!("Test4: {:?}", test.position.get(&BoardPos { x: 1, y: 6 }).unwrap());

    // let white_rook = test.position.get(&0u16).unwrap();
    // println!("{:?}", white_rook)
    // let test2 = test.position.get(&Team::White).unwrap().get(&Pieces::Pawn).unwrap();

}
