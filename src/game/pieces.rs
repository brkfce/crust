#[allow(dead_code)]

pub struct Pieces {
    // pieces are named as [COLOUR][PIECE][STARTING COLUMN]
    // e.g. the white queen is WQD
    WRA: Rook,
    WNB: Knight,
    WBC: Bishop,
    WQD: Queen,
    WKE: King,
    WBF: Bishop,
    WNG: Knight,
    WRH: Rook,
    WPA: Pawn,
    WPB: Pawn,
    WPC: Pawn,
    WPD: Pawn,
    WPE: Pawn,
    WPF: Pawn,
    WPG: Pawn,
    WPH: Pawn,
    BPA: Pawn,
    BPB: Pawn,
    BPC: Pawn,
    BPD: Pawn,
    BPE: Pawn,
    BPF: Pawn,
    BPG: Pawn,
    BPH: Pawn,
    BRA: Rook,
    BNB: Knight,
    BBC: Bishop,
    BQD: Queen,
    BKE: King,
    BBF: Bishop,
    BNG: Knight,
    BRH: Rook,
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
    let WRA = Rook {
        colour: Colour::White,
        column: Column::A,
        row: 0,
    };
    let WNB = Knight {
        colour: Colour::White,
        column: Column::B,
        row: 0,
    };
    let WBC = Bishop {
        colour: Colour::White,
        column: Column::C,
        row: 0,
    };
    let WQD = Queen {
        colour: Colour::White,
        column: Column::D,
        row: 0,
    };
    let WKE = King {
        colour: Colour::White,
        column: Column::E,
        row: 0,
    };
    let WBF = Bishop {
        colour: Colour::White,
        column: Column::F,
        row: 0,
    };
    let WNG = Knight {
        colour: Colour::White,
        column: Column::G,
        row: 0,
    };
    let WRH = Rook {
        colour: Colour::White,
        column: Column::H,
        row: 0,
    };
    let WPA = Pawn {
        colour: Colour::White,
        column: Column::A,
        row: 1,
    };
    let WPB = Pawn {
        colour: Colour::White,
        column: Column::B,
        row: 1,
    };
    let WPC = Pawn {
        colour: Colour::White,
        column: Column::C,
        row: 1,
    };
    let WPD = Pawn {
        colour: Colour::White,
        column: Column::D,
        row: 1,
    };
    let WPE = Pawn {
        colour: Colour::White,
        column: Column::E,
        row: 1,
    };
    let WPF = Pawn {
        colour: Colour::White,
        column: Column::F,
        row: 1,
    };
    let WPG = Pawn {
        colour: Colour::White,
        column: Column::G,
        row: 1,
    };
    let WPH = Pawn {
        colour: Colour::White,
        column: Column::H,
        row: 1,
    };
    let BPA = Pawn {
        colour: Colour::Black,
        column: Column::A,
        row: 6,
    };
    let BPB = Pawn {
        colour: Colour::Black,
        column: Column::B,
        row: 6,
    };
    let BPC = Pawn {
        colour: Colour::Black,
        column: Column::C,
        row: 6,
    };
    let BPD = Pawn {
        colour: Colour::Black,
        column: Column::D,
        row: 6,
    };
    let BPE = Pawn {
        colour: Colour::Black,
        column: Column::E,
        row: 6,
    };
    let BPF = Pawn {
        colour: Colour::Black,
        column: Column::F,
        row: 6,
    };
    let BPG = Pawn {
        colour: Colour::Black,
        column: Column::G,
        row: 6,
    };
    let BPH = Pawn {
        colour: Colour::Black,
        column: Column::H,
        row: 6,
    };
    let BRA = Rook {
        colour: Colour::Black,
        column: Column::A,
        row: 7,
    };
    let BNB = Knight {
        colour: Colour::Black,
        column: Column::B,
        row: 7,
    };
    let BBC = Bishop {
        colour: Colour::Black,
        column: Column::C,
        row: 7,
    };
    let BQD = Queen {
        colour: Colour::Black,
        column: Column::D,
        row: 7,
    };
    let BKE = King {
        colour: Colour::Black,
        column: Column::E,
        row: 7,
    };
    let BBF = Bishop {
        colour: Colour::Black,
        column: Column::F,
        row: 7,
    };
    let BNG = Knight {
        colour: Colour::Black,
        column: Column::G,
        row: 7,
    };
    let BRH = Rook {
        colour: Colour::Black,
        column: Column::H,
        row: 7,
    };

    let pieces = Pieces {
        WRA: WRA,
        WNB: WNB,
        WBC: WBC,
        WQD: WQD,
        WKE: WKE,
        WBF: WBF,
        WNG: WNG,
        WRH: WRH,
        WPA: WPA,
        WPB: WPB,
        WPC: WPC,
        WPD: WPD,
        WPE: WPE,
        WPF: WPF,
        WPG: WPG,
        WPH: WPH,
        BPA: BPA,
        BPB: BPB,
        BPC: BPC,
        BPD: BPD,
        BPE: BPE,
        BPF: BPF,
        BPG: BPG,
        BPH: BPH,
        BRA: BRA,
        BNB: BNB,
        BBC: BBC,
        BQD: BQD,
        BKE: BKE,
        BBF: BBF,
        BNG: BNG,
        BRH: BRH,
    };
    pieces
}
