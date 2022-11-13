use std::collections::HashMap;


// TODO: translation between chess moves and boardpos
#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub struct BoardPos {
  pub x: u8,
  pub y: u8,
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub enum Pieces {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub enum Team {
    White,
    Black
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Material {
    kind: Pieces,
    team: Team,
    pos: BoardPos,
    has_moved: bool,
}

impl Material {
    pub fn new(team: Team, kind: Pieces, pos: BoardPos) -> Material {
        Self {
            team,
            kind,
            pos,
            has_moved: false
        }
    }
}

type PositionHashMap = HashMap<BoardPos, Material>;



const fn build_piece(team: Team, kind: Pieces, pos: BoardPos) -> Material {
    Material {
        team,
        kind,
        pos,
        has_moved: false
    }
}

const INITIAL_BOARD: &[(BoardPos, Material)] = &[
  (BoardPos{x:0,y:0}, build_piece(Team::White, Pieces::Rook, BoardPos{x:0,y:0})),
  (BoardPos{x:1,y:0}, build_piece(Team::White, Pieces::Knight, BoardPos{x:1,y:0})),
  (BoardPos{x:2,y:0}, build_piece(Team::White, Pieces::Bishop, BoardPos{x:2,y:0})),
  (BoardPos{x:3,y:0}, build_piece(Team::White, Pieces::Queen, BoardPos{x:3,y:0})),
  (BoardPos{x:4,y:0}, build_piece(Team::White, Pieces::King, BoardPos{x:4,y:0})),
  (BoardPos{x:5,y:0}, build_piece(Team::White, Pieces::Bishop, BoardPos{x:5,y:0})),
  (BoardPos{x:6,y:0}, build_piece(Team::White, Pieces::Knight, BoardPos{x:6,y:0})),
  (BoardPos{x:7,y:0}, build_piece(Team::White, Pieces::Rook, BoardPos{x:7,y:0})),
  (BoardPos{x:0,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:0,y:1})),
  (BoardPos{x:1,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:1,y:1})),
  (BoardPos{x:2,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:2,y:1})),
  (BoardPos{x:3,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:3,y:1})),
  (BoardPos{x:4,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:4,y:1})),
  (BoardPos{x:5,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:5,y:1})),
  (BoardPos{x:6,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:6,y:1})),
  (BoardPos{x:7,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:7,y:1})),
  (BoardPos{x:0,y:1}, build_piece(Team::Black, Pieces::Rook, BoardPos{x:0,y:7})),
  (BoardPos{x:1,y:7}, build_piece(Team::Black, Pieces::Knight, BoardPos{x:1,y:7})),
  (BoardPos{x:2,y:7}, build_piece(Team::Black, Pieces::Bishop, BoardPos{x:2,y:7})),
  (BoardPos{x:3,y:7}, build_piece(Team::Black, Pieces::King, BoardPos{x:3,y:7})),
  (BoardPos{x:4,y:7}, build_piece(Team::Black, Pieces::Queen, BoardPos{x:4, y:7})),
  (BoardPos{x:5,y:7}, build_piece(Team::Black, Pieces::Bishop, BoardPos{x:5,y:7})),
  (BoardPos{x:6,y:7}, build_piece(Team::Black, Pieces::Knight, BoardPos{x:6,y:7})),
  (BoardPos{x:7,y:7}, build_piece(Team::Black, Pieces::Rook, BoardPos{x:7,y:7})),
  (BoardPos{x:0,y:7}, build_piece(Team::Black, Pieces::Pawn, BoardPos{x:0,y:6})),
  (BoardPos{x:1,y:6}, build_piece(Team::Black, Pieces::Pawn, BoardPos{x:1,y:6})),
  (BoardPos{x:2,y:6}, build_piece(Team::Black, Pieces::Pawn, BoardPos{x:2,y:6})),
  (BoardPos{x:3,y:6}, build_piece(Team::Black, Pieces::Pawn, BoardPos{x:3,y:6})),
  (BoardPos{x:4,y:6}, build_piece(Team::Black, Pieces::Pawn, BoardPos{x:4,y:6})),
  (BoardPos{x:5,y:6}, build_piece(Team::Black, Pieces::Pawn, BoardPos{x:5,y:6})),
  (BoardPos{x:6,y:6}, build_piece(Team::Black, Pieces::Pawn, BoardPos{x:6,y:6})),
  (BoardPos{x:7,y:6}, build_piece(Team::Black, Pieces::Pawn, BoardPos{x:7,y:6})),
];

fn fresh_board() -> PositionHashMap {
    let mut map: PositionHashMap = HashMap::new();
    for (pos, material) in INITIAL_BOARD.iter() {
      map.insert(*pos, *material);
    }

    map
}

pub struct Board {
  pub bitboard: u16, 
  pub position: PositionHashMap
}

impl Board {
    pub fn new() -> Self {
        let mut map: PositionHashMap = fresh_board();
        
        Self { 
            bitboard: 64,
            // position: map,
            position: fresh_board()
        }
    }

    pub fn reset_board(&mut self) {
      self.position = fresh_board();
    }

    pub fn get_piece(&self, target: BoardPos) -> Option<Pieces> {
      Some(self.position.get(&target).unwrap().kind)
    }

    pub fn move_piece(&mut self, from: BoardPos, to: BoardPos) {
        let mut cur = self.position.get(&from).unwrap().to_owned();
        // TODO: Some logic to decide whether or not the piece can actually move where it wants to go

        if Self::check_move_is_legal(self, &cur, from,  to) {
          self.position.remove(&from);
          cur.pos = to;
          if(!cur.has_moved) { cur.has_moved = true; }
          self.position.insert(to, cur);
        } else {
          // println!("Illegal Move!")
          panic!("Illegal Move")
        }
    }

    fn find_legal_moves(piece: &Material) -> Vec<BoardPos> {
      // let mut map = HashMap::new();
      let mut moves : Vec<BoardPos> = vec![];
      match piece.kind {
        Pieces::Pawn => {
          println!("test");
          match piece.team {
            Team::White => {
              if piece.has_moved {
                todo!()
                // in this case, pawn can only move one up, unless en passante is available, or blocked
              } else {
                // map.in
                moves.push(BoardPos {x: piece.pos.x, y: piece.pos.y + 1});
                moves.push(BoardPos { x: piece.pos.x, y: piece.pos.y + 2 });
              }
            },
            Team::Black => todo!(),
          }
        }
        Pieces::Rook => todo!(),
        Pieces::Knight => todo!(),
        Pieces::Bishop => todo!(),
        Pieces::Queen => todo!(),
        Pieces::King => todo!(),
      }
      // map
      moves
    }

    fn check_move_is_legal(&mut self, piece: &Material, from: BoardPos, to: BoardPos) -> bool {
      // check that friendly pieces are not on the spot wanting to move to
      // check that move will not put king into check
      // let legal_moves = Self::find_legal_moves(&piece, &target);
      // if Self::find_legal_moves(piece, to) {
      //   todo!()
      // }
      // if Self::find_legal_moves(piece).contains(&to) {
      //   true
      // } else {
      //   false
      // }
      // Self::find_legal_moves(piece).contains(&to)
      let legal_moves = Self::find_legal_moves(piece);
      legal_moves.iter().for_each(|&val| println!("legal move: {:?}", val));
      Self::find_legal_moves(piece).iter().any(|&legal_move| legal_move == to)
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test] #[should_panic]
  fn illegal_moves_should_panic() {
    let mut test = Board::new();
    
    // try move pawn 5 spaces forward...
    test.move_piece(BoardPos { x: 1, y: 1 }, BoardPos { x: 1, y: 6});
  }

  #[test]
  fn board_resets() {
    let mut test = Board::new();
    test.move_piece(BoardPos { x: 1, y: 1 }, BoardPos { x: 1, y: 2});
    assert_eq!(Pieces::Pawn, test.get_piece(BoardPos {x: 1, y: 2}).unwrap());
    test.reset_board();
    assert_eq!(Pieces::Pawn, test.get_piece(BoardPos {x: 1, y: 1}).unwrap());
  }

}