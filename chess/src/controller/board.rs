use std::collections::HashMap;

/**
 * TODO list
 * TODO: refactor build_bp to not take a tuple, this would include calls to this fn..
 */

type MovesVec = Vec<BoardPos>;

pub fn build_bp(bp: (i16, i16)) -> BoardPos {
  BoardPos {
    x: bp.0,
    y: bp.1
  }
}

// TODO: translation between chess moves and boardpos
#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub struct BoardPos {
  pub x: i16,
  pub y: i16,
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

  // white back row
  (BoardPos{x:0,y:0}, build_piece(Team::White, Pieces::Rook, BoardPos{x:0,y:0})),
  (BoardPos{x:1,y:0}, build_piece(Team::White, Pieces::Knight, BoardPos{x:1,y:0})),
  (BoardPos{x:2,y:0}, build_piece(Team::White, Pieces::Bishop, BoardPos{x:2,y:0})),
  (BoardPos{x:3,y:0}, build_piece(Team::White, Pieces::Queen, BoardPos{x:3,y:0})),
  (BoardPos{x:4,y:0}, build_piece(Team::White, Pieces::King, BoardPos{x:4,y:0})),
  (BoardPos{x:5,y:0}, build_piece(Team::White, Pieces::Bishop, BoardPos{x:5,y:0})),
  (BoardPos{x:6,y:0}, build_piece(Team::White, Pieces::Knight, BoardPos{x:6,y:0})),
  (BoardPos{x:7,y:0}, build_piece(Team::White, Pieces::Rook, BoardPos{x:7,y:0})),

  // white pawns
  (BoardPos{x:0,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:0,y:1})),
  (BoardPos{x:1,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:1,y:1})),
  (BoardPos{x:2,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:2,y:1})),
  (BoardPos{x:3,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:3,y:1})),
  (BoardPos{x:4,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:4,y:1})),
  (BoardPos{x:5,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:5,y:1})),
  (BoardPos{x:6,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:6,y:1})),
  (BoardPos{x:7,y:1}, build_piece(Team::White, Pieces::Pawn, BoardPos{x:7,y:1})),

  // black back row
  (BoardPos{x:0,y:1}, build_piece(Team::Black, Pieces::Rook, BoardPos{x:0,y:7})),
  (BoardPos{x:1,y:7}, build_piece(Team::Black, Pieces::Knight, BoardPos{x:1,y:7})),
  (BoardPos{x:2,y:7}, build_piece(Team::Black, Pieces::Bishop, BoardPos{x:2,y:7})),
  (BoardPos{x:3,y:7}, build_piece(Team::Black, Pieces::King, BoardPos{x:3,y:7})),
  (BoardPos{x:4,y:7}, build_piece(Team::Black, Pieces::Queen, BoardPos{x:4, y:7})),
  (BoardPos{x:5,y:7}, build_piece(Team::Black, Pieces::Bishop, BoardPos{x:5,y:7})),
  (BoardPos{x:6,y:7}, build_piece(Team::Black, Pieces::Knight, BoardPos{x:6,y:7})),
  (BoardPos{x:7,y:7}, build_piece(Team::Black, Pieces::Rook, BoardPos{x:7,y:7})),

  // black pawns
  (BoardPos{x:0,y:6}, build_piece(Team::Black, Pieces::Pawn, BoardPos{x:0,y:6})),
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
        Self { 
            bitboard: 64,
            position: fresh_board()
        }
    }

    pub fn reset_board(&mut self) {
      self.position = fresh_board();
    }

    pub fn get_piece(&self, target: BoardPos) -> Option<&Material> {
      self.position.get(&target)
    }

    pub fn move_piece(&mut self, from: BoardPos, to: BoardPos) {
        let mut cur = self.position.get(&from).unwrap().to_owned();
       
        if Self::check_move_is_legal(self, &cur, from,  to) {
          self.position.remove(&from);
          cur.pos = to;
          if !cur.has_moved { cur.has_moved = true; }
          self.position.insert(to, cur);
        } else {
          panic!("Illegal Move")
        }
    }

    fn check_bounds(pos: BoardPos) -> Option<BoardPos> {
      if pos.x > 7 || pos.y > 7 || pos.x < 0 || pos.y < 0 { None }
      else { Some(pos) }
    }

    fn check_pawn_is_blocked(&self, pos: BoardPos, target: BoardPos) -> Option<BoardPos> {
      let targ = target;
      match self.get_piece(targ) {
        Some(i) => {
          None
        },
        _ => Some(target)
      }
    }

    fn check_bounds_and_blocked(&self, pos: BoardPos, target: BoardPos ) -> Option<BoardPos> {
      match Self::check_bounds(target) {
        Some(_) => {},
        None => { return None }
      }
      
      match Self::check_pawn_is_blocked(self, pos, target) {
        Some(targ) => { Some(targ) },
        None => { None }
      }
    }

    fn pawn_eats(&self, piece: &Material) -> Vec<Option<BoardPos>> {
      let mut eats: Vec<Option<BoardPos>> = vec![];
      match piece.team {
        Team::Black => {
          match self.get_piece(BoardPos { x: piece.pos.x + 1, y: piece.pos.y - 1}) {
            Some(_) => {
              eats.push(Some(BoardPos { x: piece.pos.x + 1, y: piece.pos.y - 1}));
            },
            None => {},
          }
          match self.get_piece(BoardPos { x: piece.pos.x - 1, y: piece.pos.y - 1}) {
            Some(_) => {
              eats.push(Some(BoardPos { x: piece.pos.x - 1, y: piece.pos.y - 1}));
            },
            None => {},
          }
        },
        Team::White => {
          match self.get_piece(BoardPos { x: piece.pos.x + 1, y: piece.pos.y + 1}) {
            Some(_) => {
              eats.push(Some(BoardPos { x: piece.pos.x + 1, y: piece.pos.y + 1}));
            },
            None => {},
          }
          match self.get_piece(BoardPos { x: piece.pos.x - 1, y: piece.pos.y + 1}) {
            Some(_) => {
              eats.push(Some(BoardPos { x: piece.pos.x - 1, y: piece.pos.y + 1}));
            },
            None => {},
          }
        },
      }
      eats
    }

    fn try_add_pawn_move(&self, moves: &mut MovesVec, piece: &Material, target: BoardPos) {
      match Self::check_bounds_and_blocked(&self, piece.pos, target) {
        Some(targ) => moves.push(targ),
        None => {}
      }
    }

    fn find_legal_pawn_moves<'a>(&self, piece: &Material, moves: &'a mut Vec<BoardPos>) -> &'a mut Vec<BoardPos> {
      match piece.team {
        Team::White => 
        {
          Self::try_add_pawn_move(self, moves, piece, build_bp((piece.pos.x, piece.pos.y + 1)));

          let edibles = Self::pawn_eats(self, piece);
          // see if there is a piece to eat
          for edible in edibles.iter() {
            match edible {
                Some(pos) => {
                  moves.push(*pos);
                },
                None => {},
            }
          }
          if !piece.has_moved {
            Self::try_add_pawn_move(self, moves, piece, build_bp((piece.pos.x, piece.pos.y+2)));
          }
        },
        Team::Black => {
          Self::try_add_pawn_move(self, moves, piece, build_bp((piece.pos.x, piece.pos.y -1)));

          if !piece.has_moved {
            Self::try_add_pawn_move(self, moves, piece, build_bp((piece.pos.x, piece.pos.y - 2)));
          }
        },
      }
      moves
    }

    /**
     * Wrapper method to box in case multiple checks are needed
     */
    fn try_add_knight_move(&self, moves: &mut MovesVec, piece: &Material, target: BoardPos) {
      match Self::check_bounds(target) {
        Some(targ) => {
          moves.push(targ);
          dbg!("try added {:#?}", &moves);
        },
        None => {},
      }
    }

    fn find_legal_knight_moves<'a>(&self, piece: &Material, moves: &'a mut Vec<BoardPos>) -> &'a mut Vec<BoardPos> {
      let board_pos: &[(i16, i16)] = &[
        (piece.pos.x + 1, piece.pos.y + 2), // right one, up two
        (piece.pos.x + 2, piece.pos.y + 1), // right two, up one
        (piece.pos.x - 1, piece.pos.y + 2), // left one, up two
        (piece.pos.x - 2, piece.pos.y + 1), // left two, up one
        (piece.pos.x + 1, piece.pos.y - 2), // right one, down two
        (piece.pos.x + 2, piece.pos.y - 1), // right two, down one
        (piece.pos.x - 1, piece.pos.y - 2), // left 1, down two
        (piece.pos.x - 2, piece.pos.y - 1), // left two, down
      ];

      for (x, y) in board_pos.iter() {
        Self::try_add_knight_move(&self, moves, piece, build_bp((*x, *y)));
      }
      dbg!("Legal Knight Moves: {:#?}", &moves);

      moves
    }

    fn rook_eats(&self, piece: &Material, target: BoardPos) -> Option<BoardPos> {
      Some(piece.pos)

    }

    fn rook_check_bounds_and_eat(&self, target: BoardPos, piece: &Material) -> Option<BoardPos>{
      match Self::rook_eats(self, piece, target) {
        Some(targ) => {
          // Some(targ)

        },
        None => {},
      }
      match Self::check_bounds(target) {
        Some(targ) => {
          Some(targ)
        },
        None => {return None},
      }


    }

    fn find_legal_rook_moves<'a>(&self, piece: &Material, moves: &'a mut Vec<BoardPos>) -> &'a mut Vec<BoardPos> {
      for i in piece.pos.y .. 8 {
        // check upward direction
        match Self::rook_check_bounds_and_eat(self, build_bp((piece.pos.x, piece.pos.y + i)), piece) {
            Some(targ) => { moves.push(targ); },
            None => break,
        }
      }

      moves
    }


    fn find_legal_moves(&self, piece: &Material) -> Vec<BoardPos> {
      match piece.kind {
        Pieces::Pawn => { 
          let mut moves : Vec<BoardPos> = vec![];
          return Self::find_legal_pawn_moves(self, piece, &mut moves).to_vec()
        },
        Pieces::Rook => {
          let mut moves : Vec<BoardPos> = vec![];
          return Self::find_legal_pawn_moves(self, piece, &mut moves).to_vec()
        },
        Pieces::Knight => { 
          let mut moves : Vec<BoardPos> = vec![];
          return Self::find_legal_knight_moves(self, piece, &mut moves).to_vec()
        },
        Pieces::Bishop => todo!(),
        Pieces::Queen => todo!(),
        Pieces::King => todo!(),
      }
    }

    /**
     * Check move complies with movement limitations of certain piece type
     * Check friendly pieces are not on the spot wanting to move to
     * Check move will not put king in check
     */
    fn check_move_is_legal(&mut self, piece: &Material, from: BoardPos, to: BoardPos) -> bool {
      // check that friendly pieces are not on the spot wanting to move to
      // check that move will not put king into check
      Self::find_legal_moves(self, piece).iter().any(|&legal_move| legal_move == to)
    }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn board_resets() {
    let mut test = Board::new();
    test.move_piece(BoardPos { x: 1, y: 1 }, BoardPos { x: 1, y: 2});
    assert_eq!(Pieces::Pawn, test.get_piece(BoardPos {x: 1, y: 2}).unwrap().kind);
    test.reset_board();
    assert_eq!(Pieces::Pawn, test.get_piece(BoardPos {x: 1, y: 1}).unwrap().kind);
  }

  #[test]
  fn pawn_can_move_one_space() {
    let mut board = Board::new();

    board.move_piece(BoardPos {x: 1, y: 1}, BoardPos {x: 1, y: 2});
    assert_eq!(Pieces::Pawn, board.get_piece(BoardPos {x: 1, y: 2}).unwrap().kind);
  }

  #[test]
  fn pawn_can_eat() {
    let mut board = Board::new();
    board.move_piece(BoardPos {x: 1, y: 1}, BoardPos {x: 1, y: 3});
    board.move_piece(BoardPos {x: 2, y: 6}, BoardPos {x: 2, y: 4});
    board.move_piece(BoardPos {x: 1, y: 3}, BoardPos {x: 2, y: 4});
  }

  #[test] #[should_panic]
  fn pawn_moving_through_piece_panics() {
    let mut board = Board::new();
    board.move_piece(BoardPos {x: 1, y: 1}, BoardPos { x: 1, y: 3 });
    board.move_piece(BoardPos { x: 1, y: 6 }, BoardPos{x: 1, y: 4});

    board.move_piece(BoardPos { x: 1, y: 3 }, BoardPos {x: 1, y: 4});
  }

  #[test] #[should_panic]
  fn move_pawn_five_spaces_panics() {
    let mut test = Board::new();
    
    // try move pawn 5 spaces forward...
    test.move_piece(BoardPos { x: 1, y: 1 }, BoardPos { x: 1, y: 6});

    // try move 
  }

  #[test] #[should_panic]
  fn pawn_going_out_of_bounds_panics() {
    let mut test = Board::new();

    test.move_piece(BoardPos { x: 1, y: 1 }, BoardPos { x: 1, y: 2});
    test.move_piece(BoardPos { x: 1, y: 2 }, BoardPos { x: 1, y: 3});
    test.move_piece(BoardPos { x: 1, y: 3 }, BoardPos { x: 1, y: 4});
    test.move_piece(BoardPos { x: 1, y: 4 }, BoardPos { x: 1, y: 5});
    test.move_piece(BoardPos { x: 1, y: 5 }, BoardPos { x: 1, y: 6});
    test.move_piece(BoardPos { x: 1, y: 6 }, BoardPos { x: 1, y: 7});
    test.move_piece(BoardPos { x: 1, y: 7 }, BoardPos { x: 1, y: 8});
    test.move_piece(BoardPos { x: 1, y: 8 }, BoardPos { x: 1, y: 9});
  }
  /**
   * Rook Tests
   */

  #[test]
  fn rook_basic_moves() {
    let mut board = Board::new();

    let moves: &[(((i16, i16),(i16, i16)), ((i16, i16),(i16, i16)))] = &[
      (((0, 1), (0, 2)), ((0, 6), (0, 4))),
      (((0,0), (0, 1)), ((1,6), (1,4))),
    ];

    for (white, black) in moves.iter() {
      board.move_piece(build_bp(white.0), build_bp(white.1));
      board.move_piece(build_bp(black.0), build_bp(black.1));
    }
  }

  /**
   * Knight Tests
   */
  #[test]
  fn knight_can_move() {
    let mut test = Board::new();
    let piece = Material::new(Team::White, Pieces::Knight, build_bp((1, 0)));

    let knight_moves: &[(i16, i16)] = &[
        (piece.pos.x + 1, piece.pos.y + 2), // right one, up two
        (piece.pos.x + 2, piece.pos.y + 1), // right two, up one
        (piece.pos.x - 1, piece.pos.y + 2), // left one, up two
      ];


    for (x, y) in knight_moves.iter() {
      test.move_piece(build_bp((1, 0)), build_bp((*x, *y)));
      assert_eq!(Pieces::Knight, test.get_piece(build_bp((*x,*y))).unwrap().kind);
      test.reset_board();
    }
  }

  #[test]
  fn knight_can_eat() {
    let mut board = Board::new();

    let moves: &[(((i16, i16),(i16, i16)), ((i16, i16),(i16, i16)))] = &[
      (((1,0),(2, 2)), ((3,6),(3,4))),
      (((2,2),(3, 4)), ((4,6),(4, 4))),
    ];

    for (white, black) in moves.iter() {
      board.move_piece(build_bp(white.0), build_bp(white.1));
      dbg!("Knight pos after move: {:?}\tGet Piece after Move: {:?}", white.0, board.get_piece(build_bp(white.1)).unwrap());
      board.move_piece(build_bp(black.0), build_bp(black.1));
    }

    assert_eq!(Pieces::Knight, board.get_piece(build_bp((3,4))).unwrap().kind)
  }

  #[test] #[should_panic]
  fn illegal_knight_moves() {
    let mut test = Board::new();
    let piece = Material::new(Team::White, Pieces::Knight, build_bp((1, 0)));

    let knight_moves: &[(i16, i16)] = &[
        (piece.pos.x - 2, piece.pos.y + 1), // left two, up one TODO: should panic
        (piece.pos.x + 1, piece.pos.y - 2), // right one, down two TODO: should panic
        (piece.pos.x + 2, piece.pos.y - 1), // right two, down one TODO: should panic
        (piece.pos.x - 1, piece.pos.y - 2), // left 1, down two TODO: should panic
        (piece.pos.x - 2, piece.pos.y - 1), // left two, down TODO: should panic
      ];


    for (x, y) in knight_moves.iter() {
      test.move_piece(build_bp((1, 0)), build_bp((*x, *y)));
      assert_eq!(Pieces::Knight, test.get_piece(build_bp((*x,*y))).unwrap().kind);
      test.reset_board();
    }
  }


}