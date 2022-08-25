mod board;

struct Game {
    board: board::Board,
}
// create default game
fn new_game() -> Game {
    let new_board = board::new_board();
    Game { board: new_board }
}
