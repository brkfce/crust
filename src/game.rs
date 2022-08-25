mod board;

pub struct Game {
    board: board::Board,
}
// create default game
pub fn new_game() -> Game {
    let new_board = board::new_board();
    Game { board: new_board }
}
