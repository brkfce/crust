#[allow(dead_code)]

pub struct Pawn {
    position_index: i8,
}
pub struct Knight {
    position_index: i8,
}
pub struct Bishop {
    position_index: i8,
}
pub struct Rook {
    position_index: i8,
    // for determining which side castling is still available
    kingside: bool,
}
pub struct Queen {
    position_index: i8,
}
pub struct King {
    position_index: i8,
}
