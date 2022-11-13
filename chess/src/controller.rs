mod board;
pub use board::{Board, Team, Pieces};

pub struct ChessController {
  board: Board,
}

impl ChessController {
  pub fn new() -> Self {
    Self {
      board: Board::new(),
    }
  }

  pub fn start_game(&self) {
    todo!("Game loop to be created")
    // while loop
  }
}