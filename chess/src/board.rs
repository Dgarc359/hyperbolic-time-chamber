use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub enum Pieces {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Eq, Hash, PartialEq)]
pub enum Team {
    White,
    Black
}

struct Material {
    starting_pos: u16,
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
            starting_pos: start_pos,
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
    pub bitboard: u16, // 2d array where outer arr is a col (file), inner is a row (num), ex, a2 would be arr[0][1]
    // pub position: HashMap<Team, HashMap<Pieces,Vec<u16>>>, // some structure to hold position of pieces
    pub position: HashMap<Team, Vec<GamePiece>>
}
// todo: use u16s as hashmap keys


impl Board {
    pub fn new() -> Self {
        let pos = Board::starting_pos();
        Self { 
            bitboard: 64,
            position: pos,
        }
    }
    /* There will be multiple of the same piece in diff squares, so we hold individual pos in a vec */
    pub fn starting_pos() -> HashMap<Team, HashMap<Pieces, Vec<u16>>> {
        // todo: I should probably use GamePiece instead of a nested hashmap
        // let mut white_piece_pos: HashMap<Pieces, Vec<u16>> = HashMap::new();
        // white_piece_pos.insert(Pieces::Pawn, vec![8,9,10,11,12,13,14,15]);
        // white_piece_pos.insert(Pieces::Rook, vec![0, 7]);
        // white_piece_pos.insert(Pieces::Knight, vec![1,6]);
        // white_piece_pos.insert(Pieces::Bishop, vec![2, 5]);
        // white_piece_pos.insert(Pieces::Queen, vec![3]);
        // white_piece_pos.insert(Pices::King, vec![4]);
        let white_initial_pieces = vec![

        ];
        
        let mut pos: HashMap<Team, HashMap<Pieces, Vec<u16>>> = HashMap::new();
        pos.insert(Team::White, white_piece_pos);
        pos
    }
}