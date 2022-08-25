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

pub enum Colour {
    White,
    Black,
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
        white_knights: vec![
            pieces::Knight { position_index: 1 },
            pieces::Knight { position_index: 6 },
        ],
        white_bishops: vec![
            pieces::Bishop { position_index: 2 },
            pieces::Bishop { position_index: 5 },
        ],
        white_rooks: vec![
            pieces::Rook {
                position_index: 0,
                kingside: false,
            },
            pieces::Rook {
                position_index: 7,
                kingside: true,
            },
        ],
        white_queens: vec![pieces::Queen { position_index: 3 }],
        white_king: pieces::King { position_index: 4 },
        black_pawns: vec![
            pieces::Pawn { position_index: 48 },
            pieces::Pawn { position_index: 49 },
            pieces::Pawn { position_index: 50 },
            pieces::Pawn { position_index: 51 },
            pieces::Pawn { position_index: 52 },
            pieces::Pawn { position_index: 53 },
            pieces::Pawn { position_index: 54 },
            pieces::Pawn { position_index: 55 },
        ],
        black_knights: vec![
            pieces::Knight { position_index: 57 },
            pieces::Knight { position_index: 62 },
        ],
        black_bishops: vec![
            pieces::Bishop { position_index: 58 },
            pieces::Bishop { position_index: 61 },
        ],
        black_rooks: vec![
            pieces::Rook {
                position_index: 56,
                kingside: false,
            },
            pieces::Rook {
                position_index: 63,
                kingside: true,
            },
        ],
        black_queens: vec![pieces::Queen { position_index: 60 }],
        black_king: pieces::King { position_index: 61 },
        move_count: 0,
        enpassant: false,
        // true if castling is still possible i.e. the rook and king have not moved
        white_castle_kingside: true,
        white_castle_queenside: true,
        black_castle_kingside: true,
        black_castle_queenside: true,
    }
}
