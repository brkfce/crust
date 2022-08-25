#[allow(dead_code)]

pub trait GenMoves {
    fn gen_moves(
        &self,
        colour: super::Colour,
        board: &super::Board,
        white_pieces: [bool; 64],
        black_pieces: [bool; 64],
    ) -> Vec<super::Board>;
}

pub struct Pawn {
    pub position_index: i8,
}
pub struct Knight {
    pub position_index: i8,
}
pub struct Bishop {
    pub position_index: i8,
}
pub struct Rook {
    pub position_index: i8,
    // for determining which side castling is still available
    pub kingside: bool,
}
pub struct Queen {
    pub position_index: i8,
}
pub struct King {
    pub position_index: i8,
}
