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
    pub bitboard: u16, // 2d array where outer arr is a col (file), inner is a row (num), ex, a2 would be arr[0][1]
    // pub position: HashMap<Team, HashMap<Pieces,Vec<u16>>>, // some structure to hold position of pieces
    // pub position: HashMap<Team, Vec<GamePiece>>
    pub position: HashMap<u16, Option<Material>>
}
// todo: use u16s as hashmap keys

fn build_piece(team: Team, cur_pos: u16, kind: Pieces) -> Material {
    Material {
        team: team,
        cur_pos,
        kind: kind
    }
}

// impl Material {
//     pub fn move() {

//     }
// }

fn build_board() -> HashMap<u16, Material> {
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
        // let pos = Board::starting_pos();
        let mut map: HashMap<u16, Material> = build_board();
        
        Self { 
            bitboard: 64,
            position: map,
        }
    }

    pub fn move_piece(&self, from: u16, to: u16) {
        let cur = self.position.get(&from).unwrap().unwrap();
        self.position.insert(from, Some(None));
        self.position.insert(to, Some(cur));
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