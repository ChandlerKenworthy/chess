use std::ops::Not;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    White = 0,
    Black = 1
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Castling {
    WhiteQueenside = 0,
    WhiteKingside = 1,
    BlackQueenside = 2,
    BlackKingside = 3
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5
}

impl TryFrom<u32> for PieceType {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Pawn),
            1 => Ok(Self::Knight),
            2 => Ok(Self::Bishop),
            3 => Ok(Self::Rook),
            4 => Ok(Self::Queen),
            5 => Ok(Self::King),
            _ => Err(()),
        }
    }
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

    pub fn new_from_index(index: usize) -> Self {
        assert!(index < 12);
        // Absolutely unsafe, should probably change
        Self::ALL[index]
    }

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