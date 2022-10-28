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
    // pub position: HashMap<Team, Vec<GamePiece>>
    pub position: HashMap<u16, Material>
}
// todo: use u16s as hashmap keys

fn build_piece(team: Team, sp: u16, kind: Pieces) -> Material {
    Material {
        team: team,
        starting_pos: sp,
        kind: kind
    }
}

fn build_board() -> HashMap<u16, Material> {
    let mut map: HashMap<u16, Material> = HashMap::new();

    /* White Pieces */

    // Rooks
    map.insert(0, build_piece(Team::White, 0, Pieces::Rook));
    map.insert(7, build_piece(Team::White, 7, Pieces::Rook));

    // Knights
    map.insert(1, build_piece(Team::White, 1, Pieces::Knight));
    map.insert(6, build_piece(Team::White, 6, Pieces::Knight));

    // Bishops
    map.insert(2, build_piece(Team::White, 2, Pieces::Bishop));
    map.insert(5, build_piece(Team::White, 5, Pieces::Bishop));

    // Queen
    map.insert(3, build_piece(Team::White, 3, Pieces::Queen));

    // King
    map.insert(4, build_piece(Team::White, 4, Pieces::King));

    // Pawns
    map.insert(8, build_piece(Team::White, 8, Pieces::Pawn));
    map.insert(9, build_piece(Team::White, 9, Pieces::Pawn));
    map.insert(10, build_piece(Team::White, 10, Pieces::Pawn));
    map.insert(11, build_piece(Team::White, 11, Pieces::Pawn));
    map.insert(12, build_piece(Team::White, 12, Pieces::Pawn));
    map.insert(13, build_piece(Team::White, 13, Pieces::Pawn));
    map.insert(14, build_piece(Team::White, 14, Pieces::Pawn));
    map.insert(15, build_piece(Team::White, 15, Pieces::Pawn));
    



    /* Black Pieces */
    map.insert(64, build_piece(Team::Black, 64, Pieces::Rook));
    map.insert(56, build_piece(Team::Black, 56, Pieces::Rook));

    map
}

impl Board {
    pub fn new() -> Self {
        // let pos = Board::starting_pos();
        let mut map: HashMap<u16, Material> = build_board();
        
        Self { 
            bitboard: 64,
            position: map,
        }
    }
    /* There will be multiple of the same piece in diff squares, so we hold individual pos in a vec */
    // pub fn starting_pos() -> HashMap<Team, HashMap<Pieces, Vec<u16>>> {
    //     // todo: I should probably use GamePiece instead of a nested hashmap
    //     // let mut white_piece_pos: HashMap<Pieces, Vec<u16>> = HashMap::new();
    //     // white_piece_pos.insert(Pieces::Pawn, vec![8,9,10,11,12,13,14,15]);
    //     // white_piece_pos.insert(Pieces::Rook, vec![0, 7]);
    //     // white_piece_pos.insert(Pieces::Knight, vec![1,6]);
    //     // white_piece_pos.insert(Pieces::Bishop, vec![2, 5]);
    //     // white_piece_pos.insert(Pieces::Queen, vec![3]);
    //     // white_piece_pos.insert(Pices::King, vec![4]);
    //     // let white_initial_pieces = vec![

    //     // ];
        
    //     // let mut pos: HashMap<Team, HashMap<Pieces, Vec<u16>>> = HashMap::new();
    //     // pos.insert(Team::White, white_piece_pos);
    //     // pos
    // }
}