use std::vec;

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
    white_pawns: Vec<pieces::Pawn>,
    white_knights: Vec<pieces::Knight>,
    white_bishops: Vec<pieces::Bishop>,
    white_rooks: Vec<pieces::Rook>,
    white_queens: Vec<pieces::Queen>,
    white_king: pieces::King,
    black_pawns: Vec<pieces::Pawn>,
    black_knights: Vec<pieces::Knight>,
    black_bishops: Vec<pieces::Bishop>,
    black_rooks: Vec<pieces::Rook>,
    black_queens: Vec<pieces::Queen>,
    black_king: pieces::King,
    move_count: i32,
    enpassant: bool,
    white_castle_kingside: bool,
    white_castle_queenside: bool,
    black_castle_kingside: bool,
    black_castle_queenside: bool,
}

pub fn new_board() -> Board {
    Board {
        white_pawns: vec![
            pieces::Pawn { position_index: 8 },
            pieces::Pawn { position_index: 9 },
            pieces::Pawn { position_index: 10 },
            pieces::Pawn { position_index: 11 },
            pieces::Pawn { position_index: 12 },
            pieces::Pawn { position_index: 13 },
            pieces::Pawn { position_index: 14 },
            pieces::Pawn { position_index: 15 },
        ],
        white_knights: vec![pieces::],
    }
}
