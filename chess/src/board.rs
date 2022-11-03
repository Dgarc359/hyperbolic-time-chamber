use std::collections::HashMap;

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub struct BoardPos {
  x: u8,
  y: u8,
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

#[derive(Copy, Clone, Debug)]
pub struct Material {
    kind: Pieces,
    team: Team,
}

impl Material {
    pub fn new(team: Team, kind: Pieces) -> Material {
        Self {
            team,
            kind,
        }
    }
    // fn calc_legal_moves(&self, team: &Team, kind: &Pieces, curr_pos: &u16) -> Vec<u16> {
    //     vec![]
    // }

    // pub fn calc_legal_moves(&self) -> Material {
    //     match self.kind {
    //         Pieces::Pawn => {
    //             let legal_moves = vec![self.current_pos + 8];
    //         },
    //         Pieces::Bishop => {
    //             // bishop legal moves
    //         },
    //         Pieces::Knight => {
    //             // knight legal moves
    //         },
    //         Pieces::Rook => {
    //             // ...
    //         },
    //         Pieces::Queen => {
    //             // ...
    //         },
    //         Pieces::King => {
    //             // ...
    //         }
    //     }
    // }
}

type PositionHashMap = HashMap<BoardPos, Material>;

pub struct Board {
    pub bitboard: u16, 
    pub position: PositionHashMap
}

const fn build_piece(team: Team, kind: Pieces) -> Material {
    Material {
        team,
        kind
    }
}

const INITIAL_BOARD: &[(BoardPos, Material)] = &[
  (BoardPos{x:0,y:0}, build_piece(Team::White, Pieces::Rook)),
  (BoardPos{x:1,y:0}, build_piece(Team::White, Pieces::Knight)),
  (BoardPos{x:2,y:0}, build_piece(Team::White, Pieces::Bishop)),
  (BoardPos{x:3,y:0}, build_piece(Team::White, Pieces::Queen)),
  (BoardPos{x:4,y:0}, build_piece(Team::White, Pieces::King)),
  (BoardPos{x:5,y:0}, build_piece(Team::White, Pieces::Bishop)),
  (BoardPos{x:6,y:0}, build_piece(Team::White, Pieces::Knight)),
  (BoardPos{x:7,y:0}, build_piece(Team::White, Pieces::Rook)),
  (BoardPos{x:0,y:1}, build_piece(Team::White, Pieces::Pawn)),
  (BoardPos{x:0,y:2}, build_piece(Team::White, Pieces::Pawn)),
  (BoardPos{x:0,y:3}, build_piece(Team::White, Pieces::Pawn)),
  (BoardPos{x:0,y:4}, build_piece(Team::White, Pieces::Pawn)),
  (BoardPos{x:0,y:5}, build_piece(Team::White, Pieces::Pawn)),
  (BoardPos{x:0,y:6}, build_piece(Team::White, Pieces::Pawn)),
  (BoardPos{x:0,y:7}, build_piece(Team::White, Pieces::Pawn)),
  (BoardPos{x:0,y:8}, build_piece(Team::White, Pieces::Pawn)),
];

fn build_board() -> PositionHashMap {
    let mut map: PositionHashMap = HashMap::new();
    for (pos, material) in INITIAL_BOARD.iter() {
      map.insert(*pos, *material);
    }

    map
}

impl Board {
    pub fn new() -> Self {
        let mut map: PositionHashMap = build_board();
        
        Self { 
            bitboard: 64,
            position: map,
        }
    }

    pub fn check_move_is_legal(&mut self, piece: &Material, to: BoardPos) -> bool {
      // check that friendly pieces are not on the spot wanting to move to
      // check that move will not put king into check

      
      false
    }

    pub fn move_piece(&mut self, from: BoardPos, to: BoardPos) {
        let cur = self.position.get(&from).unwrap().to_owned();
        // TODO: Some logic to decide whether or not the piece can actually move where it wants to go

        if Board::check_move_is_legal(self, &cur, to) {
          self.position.remove(&from);
          self.position.insert(to, cur);
        } else {
          println!("Illegal Move!")
        }
    }
}