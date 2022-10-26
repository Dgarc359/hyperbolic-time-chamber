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

struct Piece {
    legal_moves: Vec<u16>,
    current_pos: u16,
    kind: Pieces,
    team: Team,
}

fn main() {
    let mut board: u16;
}
