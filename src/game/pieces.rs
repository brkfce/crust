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
    Black = 0,
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

enum EnPassant {
    None,
    Kingside,
    Queenside,
}

pub struct Move {
    piece: PieceType,
    start_index: i8,
    end_index: i8,
    capture: bool,
    enpassant: EnPassant,
    double_pawn: bool,
}

pub trait Moves {
    fn move_list(&self, board: &super::board::Board) -> Vec<Move>;
}

struct Rook {
    colour: Colour,
    column: Column,
    row: i8,
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
