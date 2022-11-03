use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Pieces {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Team {
    White,
    Black
}

#[derive(Debug)]
pub struct Material {
    cur_pos: u16,
    kind: Pieces,
    team: Team,
}

struct GamePiece {
    material: Material,
    current_pos: u16,
    legal_moves: Vec<u16>
}

impl Material {
    pub fn new(team: Team, kind: Pieces, start_pos: u16) -> Material {
        Self {
            team: team,
            kind: kind,
            cur_pos: start_pos,
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


pub struct Board {
    pub bitboard: u16, 

    pub position: HashMap<u16, Option<Material>>
}

fn build_piece(team: Team, cur_pos: u16, kind: Pieces) -> Material {
    Material {
        team: team,
        cur_pos,
        kind: kind
    }
}

fn build_board() -> HashMap<u16, Option<Material>> {
    let mut map: HashMap<u16, Option<Material>> = HashMap::new();

    /* White Pieces */

    // Rooks
    map.insert(0, Some(build_piece(Team::White, 0, Pieces::Rook)));
    map.insert(7, Some(build_piece(Team::White, 7, Pieces::Rook)));

    // Knights
    map.insert(1, Some(build_piece(Team::White, 1, Pieces::Knight)));
    map.insert(6, Some(build_piece(Team::White, 6, Pieces::Knight)));

    // Bishops
    map.insert(2,Some(build_piece(Team::White, 2, Pieces::Bishop)));
    map.insert(5, Some(build_piece(Team::White, 5, Pieces::Bishop)));

    // Queen
    map.insert(3, Some(build_piece(Team::White, 3, Pieces::Queen)));

    // King
    map.insert(4, Some(build_piece(Team::White, 4, Pieces::King)));

    // Pawns
    map.insert(8, Some(build_piece(Team::White, 8, Pieces::Pawn)));
    map.insert(9, Some(build_piece(Team::White, 9, Pieces::Pawn)));
    map.insert(10, Some(build_piece(Team::White, 10, Pieces::Pawn)));
    map.insert(11, Some(build_piece(Team::White, 11, Pieces::Pawn)));
    map.insert(12, Some(build_piece(Team::White, 12, Pieces::Pawn)));
    map.insert(13, Some(build_piece(Team::White, 13, Pieces::Pawn)));
    map.insert(14, Some(build_piece(Team::White, 14, Pieces::Pawn)));
    map.insert(15, Some(build_piece(Team::White, 15, Pieces::Pawn)));

    /* Black Pieces */

    // rooks
    map.insert(63, Some(build_piece(Team::Black, 64, Pieces::Rook)));
    map.insert(56, Some(build_piece(Team::Black, 56, Pieces::Rook)));

    // knights
    map.insert(57, Some(build_piece(Team::Black, 57, Pieces::Knight)));
    map.insert(62, Some(build_piece(Team::Black, 63, Pieces::Knight)));

    // bishops
    map.insert(58, Some(build_piece(Team::Black, 58, Pieces::Bishop)));
    map.insert(61, Some(build_piece(Team::Black, 62, Pieces::Bishop)));

    // king
    map.insert(59, Some(build_piece(Team::Black, 59, Pieces::Bishop)));

    // queen
    map.insert(60, Some(build_piece(Team::Black, 59, Pieces::Bishop)));


    map
}

impl Board {
    pub fn new() -> Self {
        let mut map: HashMap<u16, Option<Material>> = build_board();
        
        Self { 
            bitboard: 64,
            position: map,
        }
    }

    pub fn check_move_is_legal(piece: &Material, to: u16) -> bool {

      false
    }

    pub fn move_piece(&mut self, from: u16, to: u16) {
        let cur = self.position.get(&from).unwrap().as_ref().unwrap();
        // TODO: Some logic to decide whether or not the piece can actually move where it wants to go

        if Board::check_move_is_legal(cur, to) {
          self.position.remove(&from);
          self.position.insert(to, Some(cur));
        } else {
          println!("Illegal Move!")
        }
    }
}