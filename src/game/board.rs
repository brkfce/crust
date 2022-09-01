use std::vec;

use self::pieces::GenMoves;

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

#[derive(PartialEq, Eq)]
pub enum Colour {
    White,
    Black,
}

#[derive(Clone)]
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
    enpassant_index: i8,
    white_castle_kingside: bool,
    white_castle_queenside: bool,
    black_castle_kingside: bool,
    black_castle_queenside: bool,
    white_check: bool,
    black_check: bool,
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
        enpassant_index: 64,
        // true if castling is still possible i.e. the rook and king have not moved
        white_castle_kingside: true,
        white_castle_queenside: true,
        black_castle_kingside: true,
        black_castle_queenside: true,
        // true if king is captured
        white_check: false,
        black_check: false,
    }
}

pub fn remove_piece(board: &mut Board, index: i8) {
    let wp_iter = board.white_pawns.iter();
    let wn_iter = board.white_knights.iter();
    let wb_iter = board.white_bishops.iter();
    let wr_iter = board.white_rooks.iter();
    let wq_iter = board.white_queens.iter();
    if board.white_king.position_index == index {
        board.white_check = true;
        return;
    }
    let bp_iter = board.black_pawns.iter();
    let bn_iter = board.black_knights.iter();
    let bb_iter = board.black_bishops.iter();
    let br_iter = board.black_rooks.iter();
    let bq_iter = board.black_queens.iter();
    if board.black_king.position_index == index {
        board.black_check = true;
        return;
    }

    let mut temp_vec_pos = 0;
    for pawn in wp_iter {
        if pawn.position_index == index {
            board.white_pawns.remove(temp_vec_pos);
            return;
        }
        temp_vec_pos += 1;
    }
    temp_vec_pos = 0;
    for pawn in bp_iter {
        if pawn.position_index == index {
            board.black_pawns.remove(temp_vec_pos);
            return;
        }
        temp_vec_pos += 1;
    }
    temp_vec_pos = 0;
    for knight in wn_iter {
        if knight.position_index == index {
            board.white_knights.remove(temp_vec_pos);
            return;
        }
        temp_vec_pos += 1;
    }
    temp_vec_pos = 0;
    for knight in bn_iter {
        if knight.position_index == index {
            board.black_knights.remove(temp_vec_pos);
            return;
        }
        temp_vec_pos += 1;
    }
    temp_vec_pos = 0;
    for bishop in wb_iter {
        if bishop.position_index == index {
            board.white_knights.remove(temp_vec_pos);
            return;
        }
        temp_vec_pos += 1;
    }
    temp_vec_pos = 0;
    for bishop in bb_iter {
        if bishop.position_index == index {
            board.black_knights.remove(temp_vec_pos);
            return;
        }
        temp_vec_pos += 1;
    }
    temp_vec_pos = 0;
    for rook in wr_iter {
        if rook.position_index == index {
            board.white_rooks.remove(temp_vec_pos);
            return;
        }
        temp_vec_pos += 1;
    }
    temp_vec_pos = 0;
    for rook in br_iter {
        if rook.position_index == index {
            board.black_rooks.remove(temp_vec_pos);
            return;
        }
        temp_vec_pos += 1;
    }
    temp_vec_pos = 0;
    for queen in wq_iter {
        if queen.position_index == index {
            board.white_queens.remove(temp_vec_pos);
            return;
        }
        temp_vec_pos += 1;
    }
    temp_vec_pos = 0;
    for queen in bq_iter {
        if queen.position_index == index {
            board.black_queens.remove(temp_vec_pos);
            return;
        }
        temp_vec_pos += 1;
    }
}

// generate an array that represents all the squares occupied by white pieces
fn gen_white_position(board: &Board) -> [bool; 64] {
    let mut white_positions: [bool; 64] = [false; 64];
    let wp_iter = board.white_pawns.iter();
    let wn_iter = board.white_knights.iter();
    let wb_iter = board.white_bishops.iter();
    let wr_iter = board.white_rooks.iter();
    let wq_iter = board.white_queens.iter();

    for pawn in wp_iter {
        white_positions[pawn.position_index as usize] = true;
    }
    for knight in wn_iter {
        white_positions[knight.position_index as usize] = true;
    }
    for bishop in wb_iter {
        white_positions[bishop.position_index as usize] = true;
    }
    for rook in wr_iter {
        white_positions[rook.position_index as usize] = true;
    }
    for queen in wq_iter {
        white_positions[queen.position_index as usize] = true;
    }
    white_positions[board.white_king.position_index as usize] = true;

    white_positions
}

// generate an array that represents all the squares occupied by black pieces
fn gen_black_positions(board: &Board) -> [bool; 64] {
    let mut black_positions: [bool; 64] = [false; 64];
    let bp_iter = board.black_pawns.iter();
    let bn_iter = board.black_knights.iter();
    let bb_iter = board.black_bishops.iter();
    let br_iter = board.black_rooks.iter();
    let bq_iter = board.black_queens.iter();

    for pawn in bp_iter {
        black_positions[pawn.position_index as usize] = true;
    }
    for knight in bn_iter {
        black_positions[knight.position_index as usize] = true;
    }
    for bishop in bb_iter {
        black_positions[bishop.position_index as usize] = true;
    }
    for rook in br_iter {
        black_positions[rook.position_index as usize] = true;
    }
    for queen in bq_iter {
        black_positions[queen.position_index as usize] = true;
    }
    black_positions[board.black_king.position_index as usize] = true;

    black_positions
}

pub fn gen_moves(board: &Board) {
    let white_positions = gen_white_position(board);
    let black_positions = gen_black_positions(board);
    let wp_iter = board.white_pawns.iter();
    let mut moves_list: Vec<Board> = Vec::new();
    let mut vec_pos = 0;
    for pawn in wp_iter {
        pawn.gen_moves(
            vec_pos,
            Colour::White,
            board,
            white_positions,
            black_positions,
            &mut moves_list,
        );
        vec_pos += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{gen_black_positions, gen_white_position, pieces::GenMoves};

    #[test]
    fn one_white_pawn_moves() {
        let board = super::new_board();
        let mut moves_list: Vec<super::Board> = Vec::new();
        let white_positions = gen_white_position(&board);
        let black_positions = gen_black_positions(&board);
        board.white_pawns[0].gen_moves(
            0,
            super::Colour::White,
            &board,
            white_positions,
            black_positions,
            &mut moves_list,
        );
        let move_one_index = moves_list[0].white_pawns[0].position_index;
        let move_two_index = moves_list[1].white_pawns[0].position_index;
        assert_eq!(move_one_index, 16);
        assert_eq!(move_two_index, 24);
        assert_eq!(moves_list[1].enpassant_index, 16);
    }
    #[test]
    fn one_black_pawn_moves() {
        let board = super::new_board();
        let mut moves_list: Vec<super::Board> = Vec::new();
        let white_positions = gen_white_position(&board);
        let black_positions = gen_black_positions(&board);
        board.black_pawns[0].gen_moves(
            0,
            super::Colour::Black,
            &board,
            white_positions,
            black_positions,
            &mut moves_list,
        );
        let move_one_index = moves_list[0].black_pawns[0].position_index;
        let move_two_index = moves_list[1].black_pawns[0].position_index;
        assert_eq!(move_one_index, 40);
        assert_eq!(move_two_index, 32);
        assert_eq!(moves_list[1].enpassant_index, 40);
    }
    #[test]
    fn knight_initial_moves() {
        let board = super::new_board();
        let mut moves_list: Vec<super::Board> = Vec::new();
        let white_positions = gen_white_position(&board);
        let black_positions = gen_black_positions(&board);
        let white_knights = board.white_knights.iter();
        let mut counter = 0;
        for knight in white_knights {
            knight.gen_moves(
                counter,
                super::Colour::White,
                &board,
                white_positions,
                black_positions,
                &mut moves_list,
            );
            counter += 1;
        }
        let black_knights = board.black_knights.iter();
        counter = 0;
        for knight in black_knights {
            knight.gen_moves(
                counter,
                super::Colour::Black,
                &board,
                white_positions,
                black_positions,
                &mut moves_list,
            );
            counter += 1;
        }
        assert_eq!(moves_list.len(), 8);
        assert_eq!(moves_list[0].white_knights[0].position_index, 18);
        assert_eq!(moves_list[1].white_knights[0].position_index, 16);
        assert_eq!(moves_list[2].white_knights[1].position_index, 23);
        assert_eq!(moves_list[3].white_knights[1].position_index, 21);
        assert_eq!(moves_list[4].black_knights[0].position_index, 42);
        assert_eq!(moves_list[5].black_knights[0].position_index, 40);
        assert_eq!(moves_list[6].black_knights[1].position_index, 47);
        assert_eq!(moves_list[7].black_knights[1].position_index, 45);
        assert_eq!(moves_list[0].white_knights[1].position_index, 6);
        assert_eq!(moves_list[1].white_knights[1].position_index, 6);
        assert_eq!(moves_list[2].white_knights[0].position_index, 1);
        assert_eq!(moves_list[3].white_knights[0].position_index, 1);
        assert_eq!(moves_list[4].black_knights[1].position_index, 62);
        assert_eq!(moves_list[5].black_knights[1].position_index, 62);
        assert_eq!(moves_list[6].black_knights[0].position_index, 57);
        assert_eq!(moves_list[7].black_knights[0].position_index, 57);
    }
}
