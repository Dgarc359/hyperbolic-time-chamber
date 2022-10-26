use std::io;
use std::io::Write;
use std::str::FromStr;
use strum_macros::EnumString;

enum Pieces {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

enum Team {
    White,
    Black
}

struct Material {
    legal_moves: Vec<u16>, // should be a function of some sort that calculates based off piece kind and positon and other pieces in the board, could return a vec of legal moves
    current_pos: u16,
    starting_pos: u16,
    kind: Pieces,
    team: Team,
}

impl Material {
    pub fn new(team: Team, kind: Pieces, start_pos: u16) -> Material {
        Self {
            team: team,
            kind: kind,
            starting_pos: start_pos,
            current_pos: start_pos,
            legal_moves: moves
        }
    }

    fn calc_legal_moves(&self, team: &Team, kind: &Pieces, curr_pos: &u16) -> Vec<u16> {
        vec![]
    }

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

struct Board {
    bitboard: u16, // 2d array where outer arr is a col (file), inner is a row (num), ex, a2 would be arr[0][1]
    // position: , // some structure to hold position of pieces
}

impl Board {
    pub fn new() -> Self {

        Self { bitboard: 64 }
    }
}

fn main() {
    let mat = Material::new(Team::White, Pieces::Pawn, 8);
    println!("current pos: {}", mat.current_pos);
    // let mut board: u16;
    // for num in 1..65 {
    //     if num % 8 == 0 {
    //         print!("{}\n",num);
    //         io::stdout().flush().unwrap();
    //     } else {
    //         print!("{}", num);
    //         io::stdout().flush().unwrap();
    //     }
    // }
}
