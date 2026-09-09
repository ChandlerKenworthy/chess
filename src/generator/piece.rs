#[repr(u8)]
pub enum Color {
    White = 0,
    Black = 1
}

#[repr(u8)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl Piece {
    pub const ALL: [Piece; 12] = [
        Piece::WhitePawn, Piece::WhiteKnight, Piece::WhiteBishop,
        Piece::WhiteRook, Piece::WhiteQueen,  Piece::WhiteKing,
        Piece::BlackPawn, Piece::BlackKnight, Piece::BlackBishop,
        Piece::BlackRook, Piece::BlackQueen,  Piece::BlackKing,
    ];

    pub fn new(color: Color, piece_type: PieceType) -> Self {
        let index = (color as usize) * 6 + (piece_type as usize);
        Self::ALL[index]
    }

    pub fn index(self) -> usize {
        self as usize
    }

    pub fn to_char(self) -> char {
        match self {
            Piece::WhitePawn   => 'P',
            Piece::WhiteKnight => 'N',
            Piece::WhiteBishop => 'B',
            Piece::WhiteRook   => 'R',
            Piece::WhiteQueen  => 'Q',
            Piece::WhiteKing   => 'K',
            Piece::BlackPawn   => 'p',
            Piece::BlackKnight => 'n',
            Piece::BlackBishop => 'b',
            Piece::BlackRook   => 'r',
            Piece::BlackQueen  => 'q',
            Piece::BlackKing   => 'k',
        }
    }
}