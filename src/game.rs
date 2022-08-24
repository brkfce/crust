pub mod board;
pub mod pieces;

pub struct Game {
    board: board::Board,
    pieces: pieces::Pieces,
}

#[allow(dead_code)]
pub fn new_game() -> Game {
    let pieces_set = create_pieces();
    let game_board = create_board();
    let new_game = Game {
        board: game_board,
        pieces: pieces_set,
    };
    new_game
}

fn create_pieces() -> pieces::Pieces {
    let pieces_set = pieces::Pieces {};
    pieces_set
}

fn create_board() -> board::Board {
    let game_board = board::Board {};
    game_board
}
