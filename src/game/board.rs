#[allow(dead_code)]
mod pieces;

enum Pieces {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
enum Turn {
    White = 1,
    Black = -1,
}

pub struct Board {
    white_pieces: Vec<Pieces>,
    black_pieces: Vec<Pieces>,
    turn: Turn,
    move_count: i32,
    enpassant: bool,
    white_castle_kingside: bool,
    white_castle_queenside: bool,
    black_castle_kingside: bool,
    black_castle_queenside: bool,
}

pub fn new_board() -> Board {}
