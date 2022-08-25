#[allow(dead_code)]

pub struct Pieces {
    // pieces are named as [COLOUR][PIECE][STARTING COLUMN]
    // e.g. the white queen is wqd
    wra: Rook,
    wnb: Knight,
    wbc: Bishop,
    wqd: Queen,
    wke: King,
    wbf: Bishop,
    wng: Knight,
    wrh: Rook,
    wpa: Pawn,
    wpb: Pawn,
    wpc: Pawn,
    wpd: Pawn,
    wpe: Pawn,
    wpf: Pawn,
    wpg: Pawn,
    wph: Pawn,
    bpa: Pawn,
    bpb: Pawn,
    bpc: Pawn,
    bpd: Pawn,
    bpe: Pawn,
    bpf: Pawn,
    bpg: Pawn,
    bph: Pawn,
    bra: Rook,
    bnb: Knight,
    bbc: Bishop,
    bqd: Queen,
    bke: King,
    bbf: Bishop,
    bng: Knight,
    brh: Rook,
}

enum Colour {
    White = 1,
    Black = -1,
}

enum Column {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

enum Castle {
    None,
    Kingside,
    Queenside,
}

pub struct Move {
    // which piece has moved
    piece: PieceType,
    // the index of the piece before the move
    start_index: i8,
    // the index of the piece after the move
    end_index: i8,
    // whether the piece captured another piece
    capture: bool,
    // whether the move was a castle
    castle: Castle,
    // whether the move was a pawn moving two spaces forward
    double_pawn: bool,
}

// note that pl stands for pseudo legal, i.e. a valid movement, but one that may be discounted if it causes check etc
pub trait Moves {
    fn pl_move_list(&self, board: &super::board::Board) -> Vec<Move>;
}

struct Rook {
    colour: Colour,
    column: Column,
    row: i8,
}
impl Moves for Rook {
    fn pl_move_list(&self, board: &super::board::Board) -> Vec<Move> {
        let mut pl_moves = Vec::new();
        // rooks can move in the 4 cardinal directions
        fn rook_move(
            piece: &Rook,
            board: &super::board::Board,
            pl_moves: &mut Vec<Move>,
            // change in the index of the piece for each move
            index_change: i8,
            // the index limit for the piece in the direction of motion
            index_limit: i8,
        ) {
            // whether a considered move leads to a capture, to stop looking after
            let mut capture: bool = false;
            let mut temp_index = piece.row * 8 + piece.column as i8;
            // whether the piece has hit the edge of the board
            while (temp_index + index_change).abs() < index_limit
				// whether the square is empty or contains an opposing piece
                && board.squares[(temp_index + index_change) as usize] as i8 * piece.colour as i8
                    <= 0
				// whether a capture has already been made, stopping moves in that direction
                && capture == false
            {
                if (board.squares[(temp_index + index_change) as usize] as i8)
                    * (piece.colour as i8)
                    < 0
                {
                    capture = true;
                }
                let pl_move = Move {
                    piece: PieceType::Rook,
                    start_index: temp_index,
                    end_index: temp_index + index_change,
                    capture: capture,
                    castle: Castle::None,
                    double_pawn: false,
                };
                if piece.colour as i8 == Colour::White as i8 {
                    board.white_castle = false;
                } else {
                    board.black_castle = false;
                }
                temp_index = temp_index + index_change;
                pl_moves.push(pl_move);
            }
        }
        // moves up
        rook_move(&self, board, &mut pl_moves, 8, 64 - 8);
		// moves down
		rook_move(&self, board, &mut pl_moves, -8, )
        moves
    }
}

struct Knight {
    colour: Colour,
    column: Column,
    row: i8,
}

struct Bishop {
    colour: Colour,
    column: Column,
    row: i8,
}

struct Queen {
    colour: Colour,
    column: Column,
    row: i8,
}

struct King {
    colour: Colour,
    column: Column,
    row: i8,
}

struct Pawn {
    colour: Colour,
    column: Column,
    row: i8,
}

pub fn create_pieces() -> Pieces {
    let wra = Rook {
        colour: Colour::White,
        column: Column::A,
        row: 0,
    };
    let wnb = Knight {
        colour: Colour::White,
        column: Column::B,
        row: 0,
    };
    let wbc = Bishop {
        colour: Colour::White,
        column: Column::C,
        row: 0,
    };
    let wqd = Queen {
        colour: Colour::White,
        column: Column::D,
        row: 0,
    };
    let wke = King {
        colour: Colour::White,
        column: Column::E,
        row: 0,
    };
    let wbf = Bishop {
        colour: Colour::White,
        column: Column::F,
        row: 0,
    };
    let wng = Knight {
        colour: Colour::White,
        column: Column::G,
        row: 0,
    };
    let wrh = Rook {
        colour: Colour::White,
        column: Column::H,
        row: 0,
    };
    let wpa = Pawn {
        colour: Colour::White,
        column: Column::A,
        row: 1,
    };
    let wpb = Pawn {
        colour: Colour::White,
        column: Column::B,
        row: 1,
    };
    let wpc = Pawn {
        colour: Colour::White,
        column: Column::C,
        row: 1,
    };
    let wpd = Pawn {
        colour: Colour::White,
        column: Column::D,
        row: 1,
    };
    let wpe = Pawn {
        colour: Colour::White,
        column: Column::E,
        row: 1,
    };
    let wpf = Pawn {
        colour: Colour::White,
        column: Column::F,
        row: 1,
    };
    let wpg = Pawn {
        colour: Colour::White,
        column: Column::G,
        row: 1,
    };
    let wph = Pawn {
        colour: Colour::White,
        column: Column::H,
        row: 1,
    };
    let bpa = Pawn {
        colour: Colour::Black,
        column: Column::A,
        row: 6,
    };
    let bpb = Pawn {
        colour: Colour::Black,
        column: Column::B,
        row: 6,
    };
    let bpc = Pawn {
        colour: Colour::Black,
        column: Column::C,
        row: 6,
    };
    let bpd = Pawn {
        colour: Colour::Black,
        column: Column::D,
        row: 6,
    };
    let bpe = Pawn {
        colour: Colour::Black,
        column: Column::E,
        row: 6,
    };
    let bpf = Pawn {
        colour: Colour::Black,
        column: Column::F,
        row: 6,
    };
    let bpg = Pawn {
        colour: Colour::Black,
        column: Column::G,
        row: 6,
    };
    let bph = Pawn {
        colour: Colour::Black,
        column: Column::H,
        row: 6,
    };
    let bra = Rook {
        colour: Colour::Black,
        column: Column::A,
        row: 7,
    };
    let bnb = Knight {
        colour: Colour::Black,
        column: Column::B,
        row: 7,
    };
    let bbc = Bishop {
        colour: Colour::Black,
        column: Column::C,
        row: 7,
    };
    let bqd = Queen {
        colour: Colour::Black,
        column: Column::D,
        row: 7,
    };
    let bke = King {
        colour: Colour::Black,
        column: Column::E,
        row: 7,
    };
    let bbf = Bishop {
        colour: Colour::Black,
        column: Column::F,
        row: 7,
    };
    let bng = Knight {
        colour: Colour::Black,
        column: Column::G,
        row: 7,
    };
    let brh = Rook {
        colour: Colour::Black,
        column: Column::H,
        row: 7,
    };

    Pieces {
        wra,
        wnb,
        wbc,
        wqd,
        wke,
        wbf,
        wng,
        wrh,
        wpa,
        wpb,
        wpc,
        wpd,
        wpe,
        wpf,
        wpg,
        wph,
        bpa,
        bpb,
        bpc,
        bpd,
        bpe,
        bpf,
        bpg,
        bph,
        bra,
        bnb,
        bbc,
        bqd,
        bke,
        bbf,
        bng,
        brh,
    }
}
