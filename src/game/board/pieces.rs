#[allow(dead_code)]

// generate pseudolegal moves
pub trait GenMoves {
    fn gen_moves(
        &self,
        vec_pos: usize,
        colour: super::Colour,
        board: &super::Board,
        white_positions: [bool; 64],
        black_positions: [bool; 64],
        moves_list: &mut Vec<super::Board>,
    );
}

#[derive(Clone)]
pub struct Pawn {
    pub position_index: i8,
}
impl GenMoves for Pawn {
    fn gen_moves(
        &self,
        vec_pos: usize,
        colour: super::Colour,
        board: &super::Board,
        white_positions: [bool; 64],
        black_positions: [bool; 64],
        moves_list: &mut Vec<super::Board>,
    ) {
        // pawn moves one square forward
        fn one_forward(
            piece: &Pawn,
            white_positions: [bool; 64],
            black_positions: [bool; 64],
            white: bool,
            board: &super::Board,
            vec_pos: usize,
            moves_list: &mut Vec<crate::game::board::Board>,
            index_change: i8,
        ) {
            if white_positions[(piece.position_index + index_change) as usize] != true
                && black_positions[(piece.position_index + index_change) as usize] != true
            {
                let mut move_board = (*board).clone();
                if white {
                    move_board.white_pawns[vec_pos].position_index =
                        piece.position_index + index_change;
                } else {
                    move_board.black_pawns[vec_pos].position_index =
                        piece.position_index + index_change;
                }
                move_board.enpassant = false;
                moves_list.push(move_board.clone());
            }
        }
        // pawn moves two squares forward
        fn two_forward(
            piece: &Pawn,
            white_positions: [bool; 64],
            black_positions: [bool; 64],
            white: bool,
            board: &super::Board,
            vec_pos: usize,
            moves_list: &mut Vec<crate::game::board::Board>,
            index_change: i8,
        ) {
            // pawn has to be on starting row and spaces ahead unobstructed
            if ((white && piece.position_index < 16) || (!white && piece.position_index > 47))
                && white_positions[(piece.position_index + index_change / 2) as usize] != true
                && white_positions[(piece.position_index + index_change) as usize] != true
                && black_positions[(piece.position_index + index_change / 2) as usize] != true
                && black_positions[(piece.position_index + index_change) as usize] != true
            {
                let mut move_board = (*board).clone();
                if white {
                    move_board.white_pawns[vec_pos].position_index =
                        piece.position_index + index_change;
                } else {
                    move_board.black_pawns[vec_pos].position_index =
                        piece.position_index + index_change;
                }
                // so an enpassant could occur on the next turn
                move_board.enpassant = true;
                moves_list.push(move_board.clone());
            }
        }
        fn capture(
            piece: &Pawn,
            white_positions: [bool; 64],
            black_positions: [bool; 64],
            white: bool,
            board: &super::Board,
            vec_pos: usize,
            moves_list: &mut Vec<crate::game::board::Board>,
        ) {
            if white {
                if black_positions[(piece.position_index + 9) as usize] {
                    let mut move_board = (*board).clone();
                    move_board.white_pawns[vec_pos].position_index = piece.position_index + 9;
                    super::remove_piece(&mut move_board, piece.position_index + 9);
                    move_board.enpassant = false;
                    moves_list.push(move_board.clone());
                }
                if black_positions[(piece.position_index + 7) as usize] {
                    let mut move_board = (*board).clone();
                    move_board.white_pawns[vec_pos].position_index = piece.position_index + 7;
                    super::remove_piece(&mut move_board, piece.position_index + 7);
                    move_board.enpassant = false;
                    moves_list.push(move_board.clone());
                }
            } else {
                if white_positions[(piece.position_index - 9) as usize] {
                    let mut move_board = (*board).clone();
                    move_board.black_pawns[vec_pos].position_index = piece.position_index - 9;
                    super::remove_piece(&mut move_board, piece.position_index - 9);
                    move_board.enpassant = false;
                    moves_list.push(move_board.clone());
                }
                if white_positions[(piece.position_index - 7) as usize] {
                    let mut move_board = (*board).clone();
                    move_board.black_pawns[vec_pos].position_index = piece.position_index - 7;
                    super::remove_piece(&mut move_board, piece.position_index - 7);
                    move_board.enpassant = false;
                    moves_list.push(move_board.clone());
                }
            }
        }
        if colour == super::Colour::White {
            one_forward(
                self,
                white_positions,
                black_positions,
                true,
                board,
                vec_pos,
                moves_list,
                8,
            );
            two_forward(
                self,
                white_positions,
                black_positions,
                true,
                board,
                vec_pos,
                moves_list,
                16,
            );
            capture(
                self,
                white_positions,
                black_positions,
                true,
                board,
                vec_pos,
                moves_list,
            );
        } else {
            one_forward(
                self,
                white_positions,
                black_positions,
                false,
                board,
                vec_pos,
                moves_list,
                -8,
            );
            two_forward(
                self,
                white_positions,
                black_positions,
                false,
                board,
                vec_pos,
                moves_list,
                -16,
            );
            capture(
                self,
                white_positions,
                black_positions,
                false,
                board,
                vec_pos,
                moves_list,
            );
        }
    }
}

#[derive(Clone)]
pub struct Knight {
    pub position_index: i8,
}

#[derive(Clone)]
pub struct Bishop {
    pub position_index: i8,
}

#[derive(Clone)]
pub struct Rook {
    pub position_index: i8,
    // for determining which side castling is still available
    pub kingside: bool,
}

#[derive(Clone)]
pub struct Queen {
    pub position_index: i8,
}

#[derive(Clone)]
pub struct King {
    pub position_index: i8,
}
