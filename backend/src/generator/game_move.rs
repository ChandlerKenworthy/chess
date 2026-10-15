use std::fmt;
use crate::generator::{Bitboard, PieceType, Castling, Square};

#[derive(Copy, Clone)]
pub struct Move(u32);
//   0-5: Index of origin square, e.g. Square::new(idx) 
//  6-11: Index of the target square
// 12-14: Piece type being moved
//    15: En-passant
//    16: Double pawn push
//    17: Capture (including en-passant)
//    18: Castling Queenside White
//    19: Castling Kingside White
//    20: Castling Queenside Black
//    21: Castling Kingside Black
//    22: Promotion to Queen
//    23: Promotion to Rook
//    24: Promotion to Knight
//    25: Promotion to Bishop
// 26-28: Piece type being captured (or zero for no piece)
// 29-32: Unused

pub const EN_PASSANT_FLAG: u32 = 1 << 15;

impl Move {
    const FROM_SHIFT: u32 = 0;
    const TO_SHIFT: u32 = 6;
    const PIECE_TYPE_SHIFT: u32 = 12;
    const SQUARE_MASK: u32 = 0b111111;
    const PIECE_TYPE_MASK: u32 = 0b111;
    const DOUBLE_PAWN_PUSH: u32 = 1 << 16;
    const IS_CAPTURE: u32 = 1 << 17;
    const IS_CASTLE_ANY: u32 = (1 << 18) | (1 << 19) | (1 << 20) | (1 << 21);
    const IS_PROMOTION_ANY: u32 = (1 << 22) | (1 << 23) | (1 << 24) | (1 << 25);
    const CAPTURED_PIECE_SHIFT: u32 = 26;

    pub fn new(from: Bitboard, to: Bitboard, origin_piece: PieceType) -> Self {
        let from: u32 = from.trailing_zeros();
        let to: u32 = to.trailing_zeros();

        Self(
            ((from) << Self::FROM_SHIFT) 
            | ((to) << Self::TO_SHIFT)
            | ((origin_piece as u32) << Self::PIECE_TYPE_SHIFT)
        )
    }

    pub fn new_capture(from: Bitboard, to: Bitboard, origin_piece: PieceType, captured: PieceType) -> Self {
        let from: u32 = from.trailing_zeros();
        let to: u32 = to.trailing_zeros();

        Self(
            ((from) << Self::FROM_SHIFT) 
            | ((to) << Self::TO_SHIFT)
            | ((origin_piece as u32) << Self::PIECE_TYPE_SHIFT)
            | Self::IS_CAPTURE
            | (((captured as u32) + 1) << Self::CAPTURED_PIECE_SHIFT)
        )
    }

    pub fn new_capture_with_flags(from: Bitboard, to: Bitboard, origin_piece: PieceType, captured: PieceType, flags: u32) -> Self {
        let from: u32 = from.trailing_zeros();
        let to: u32 = to.trailing_zeros();

        Self(
            ((from) << Self::FROM_SHIFT) 
            | ((to) << Self::TO_SHIFT)
            | ((origin_piece as u32) << Self::PIECE_TYPE_SHIFT)
            | Self::IS_CAPTURE
            | (((captured as u32) + 1) << Self::CAPTURED_PIECE_SHIFT)
            | flags
        )
    }

    pub fn new_with_flags(from: Bitboard, to: Bitboard, origin_piece: PieceType, mut flags: u32) -> Self {
        // Zero-out bits [0..14] in case something went wrong
        flags &= !0x7FFF;

        let from: u32 = from.trailing_zeros();
        let to: u32 = to.trailing_zeros();

        Self(
            ((from) << Self::FROM_SHIFT) 
            | ((to) << Self::TO_SHIFT)
            | ((origin_piece as u32) << Self::PIECE_TYPE_SHIFT)
            | flags
        )
    }

    #[inline(always)]
    pub fn is_double_pawn_push(&self) -> bool {
        (self.0 & Self::DOUBLE_PAWN_PUSH) != 0
    }

    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        (self.0 & Self::IS_CAPTURE) != 0
    }

    #[inline(always)]
    pub fn is_en_passant(&self) -> bool {
        (self.0 & EN_PASSANT_FLAG) != 0
    }

    #[inline(always)]
    pub fn is_castling(&self) -> bool {
        (self.0 & Self::IS_CASTLE_ANY) != 0
    }

    #[inline(always)]
    pub fn is_promotion(&self) -> bool {
        (self.0 & Self::IS_PROMOTION_ANY) != 0
    }

    #[inline(always)]
    pub fn castling_is(&self) -> Option<Castling> {
        let castling = (self.0 >> 18) & 0xF;
        match castling {
            1 => Some(Castling::WhiteQueenside), // Bit 18 (0b0001)
            2 => Some(Castling::WhiteKingside),  // Bit 19 (0b0010)
            4 => Some(Castling::BlackQueenside), // Bit 20 (0b0100)
            8 => Some(Castling::BlackKingside),  // Bit 21 (0b1000)
            _ => None,
        }
    }   

    #[inline(always)]
    pub fn promotes_to(&self) -> Option<PieceType> {
        let promotion = (self.0 >> 22) & 0xF;
        match promotion {
            1 => Some(PieceType::Queen),
            2 => Some(PieceType::Rook),
            4 => Some(PieceType::Knight),
            8 => Some(PieceType::Bishop),
            _ => None,
        }
    }

    pub fn print(self) {
        // Get the Square of from and to and print it
        let from: Square = Square::new(self.from_index());
        let to: Square = Square::new(self.to_index());
        if self.is_promotion() {
            if let Some(promotes_to) = self.promotes_to() {
                match promotes_to {
                    PieceType::Queen => print!("{from}{to}q"),
                    PieceType::Rook => print!("{from}{to}r"),
                    PieceType::Knight => print!("{from}{to}n"),
                    PieceType::Bishop => print!("{from}{to}b"),
                    _ => print!("{from}{to}"), // Should not happen
                }
            }
        } else {
            print!("{from}{to}"); 
        }
    }

    #[inline(always)]
    pub fn from(&self) -> Bitboard {
        1 << ((self.0 >> Self::FROM_SHIFT) & Self::SQUARE_MASK)
    }

    #[inline(always)]
    pub fn from_index(&self) -> u32 {
        (self.0 >> Self::FROM_SHIFT) & Self::SQUARE_MASK
    }
    
    #[inline(always)]
    pub fn to(&self) -> Bitboard {
        1 << ((self.0 >> Self::TO_SHIFT) & Self::SQUARE_MASK)
    }

    #[inline(always)]
    pub fn to_index(&self) -> u32 {
        (self.0 >> Self::TO_SHIFT) & Self::SQUARE_MASK
    }

    #[inline(always)]
    pub fn piece(&self) -> PieceType {
        let piece = (self.0 >> Self::PIECE_TYPE_SHIFT) & Self::PIECE_TYPE_MASK;
        PieceType::try_from(piece).unwrap()
    }

    #[inline(always)]
    pub fn captured_piece(&self) -> Option<PieceType> {
        let captured_piece = (self.0 >> Self::CAPTURED_PIECE_SHIFT) & Self::PIECE_TYPE_MASK;
        if captured_piece == 0 {
            None
        } else {
            Some(PieceType::try_from(captured_piece - 1).unwrap())
        }
    }
}

impl fmt::Binary for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Formats the underlying u32 using standard binary formatting
        fmt::Binary::fmt(&self.0, f)
    }
}