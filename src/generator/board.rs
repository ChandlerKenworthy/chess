use std::{debug_assert, ops::{Index, IndexMut}};
use crate::generator::{Bitboard, Color, bitboard::STARTING_BOARD, piece::Castling};
use super::piece::Piece;
use std::fmt;

pub struct Move(u32); // A tuple struct

impl Move {
    const FROM_SHIFT: u32 = 0;
    const TO_SHIFT: u32 = 6;

    const SQUARE_MASK: u32 = 0b111111;

    pub fn new(from: Bitboard, to: Bitboard) -> Self {
        // If using u8 could do e.g. debug_assert!(from < 64) etc.
        let from: u32 = from.trailing_zeros();
        let to: u32 = to.trailing_zeros();

        Self(
            ((from) << Self::FROM_SHIFT) 
            | ((to) << Self::TO_SHIFT)
        )
    }

    pub fn print(self) {
        // Get the Square of from and to and print it
        let from: Square = Square::new(self.from_index());
        let to: Square = Square::new(self.to_index());
        println!("{from}{to}"); 
    }

    pub fn from(&self) -> Bitboard {
        1 << ((self.0 >> Self::FROM_SHIFT) & Self::SQUARE_MASK)
    }

    pub fn from_index(&self) -> u32 {
        (self.0 >> Self::FROM_SHIFT) & Self::SQUARE_MASK
    }
    
    pub fn to(&self) -> Bitboard {
        1 << ((self.0 >> Self::TO_SHIFT) & Self::SQUARE_MASK)
    }

    pub fn to_index(&self) -> u32 {
        (self.0 >> Self::TO_SHIFT) & Self::SQUARE_MASK
    }

}

#[derive(Copy, Clone, Debug)]
pub struct Square(u8);

impl Square {
    pub const fn new(index: u32) -> Self {
        debug_assert!(index < 64);
        Self(index as u8)
    }

    pub const fn index(self) -> u8 {
        self.0
    }
}

// Chess convention uses lowercase square names (g1, c8, ...),
// which clashes with Rust's constant-naming lint. Silence it locally.
#[allow(non_upper_case_globals)]
impl Square {
    pub const a1: Square = Square::new(0);
    pub const b1: Square = Square::new(1);
    pub const c1: Square = Square::new(2);
    pub const d1: Square = Square::new(3);
    pub const e1: Square = Square::new(4);
    pub const f1: Square = Square::new(5);
    pub const g1: Square = Square::new(6);
    pub const h1: Square = Square::new(7);

    pub const a2: Square = Square::new(8);
    pub const b2: Square = Square::new(9);
    pub const c2: Square = Square::new(10);
    pub const d2: Square = Square::new(11);
    pub const e2: Square = Square::new(12);
    pub const f2: Square = Square::new(13);
    pub const g2: Square = Square::new(14);
    pub const h2: Square = Square::new(15);

    pub const a3: Square = Square::new(16);
    pub const b3: Square = Square::new(17);
    pub const c3: Square = Square::new(18);
    pub const d3: Square = Square::new(19);
    pub const e3: Square = Square::new(20);
    pub const f3: Square = Square::new(21);
    pub const g3: Square = Square::new(22);
    pub const h3: Square = Square::new(23);

    pub const a4: Square = Square::new(24);
    pub const b4: Square = Square::new(25);
    pub const c4: Square = Square::new(26);
    pub const d4: Square = Square::new(27);
    pub const e4: Square = Square::new(28);
    pub const f4: Square = Square::new(29);
    pub const g4: Square = Square::new(30);
    pub const h4: Square = Square::new(31);

    pub const a5: Square = Square::new(32);
    pub const b5: Square = Square::new(33);
    pub const c5: Square = Square::new(34);
    pub const d5: Square = Square::new(35);
    pub const e5: Square = Square::new(36);
    pub const f5: Square = Square::new(37);
    pub const g5: Square = Square::new(38);
    pub const h5: Square = Square::new(39);

    pub const a6: Square = Square::new(40);
    pub const b6: Square = Square::new(41);
    pub const c6: Square = Square::new(42);
    pub const d6: Square = Square::new(43);
    pub const e6: Square = Square::new(44);
    pub const f6: Square = Square::new(45);
    pub const g6: Square = Square::new(46);
    pub const h6: Square = Square::new(47);

    pub const a7: Square = Square::new(48);
    pub const b7: Square = Square::new(49);
    pub const c7: Square = Square::new(50);
    pub const d7: Square = Square::new(51);
    pub const e7: Square = Square::new(52);
    pub const f7: Square = Square::new(53);
    pub const g7: Square = Square::new(54);
    pub const h7: Square = Square::new(55);

    pub const a8: Square = Square::new(56);
    pub const b8: Square = Square::new(57);
    pub const c8: Square = Square::new(58);
    pub const d8: Square = Square::new(59);
    pub const e8: Square = Square::new(60);
    pub const f8: Square = Square::new(61);
    pub const g8: Square = Square::new(62);
    pub const h8: Square = Square::new(63);
}

// Let `1u64 << Square::g1` work directly.
impl std::ops::Shl<Square> for u64 {
    type Output = u64;
    fn shl(self, rhs: Square) -> u64 {
        self << rhs.0
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let file = (b'a' + self.0 % 8) as char;
        let rank = (b'1' + self.0 / 8) as char;

        write!(f, "{file}{rank}")
    }
}

pub struct Board {
    pub bitboards: [Bitboard; 12],
    // TODO: Compress into a short e.g. 32-bit word instead? 1 bit for color, 6 bits for en-passant square index, 4 for castling rights
    pub to_move: Color,
    pub en_passant: Bitboard, // the square a pawn may move to on the next move completing an en-passant capture (empty if not available)
    pub castling_rights: u8, // [White QS][White KS][Black QS][Black KS][----]
}

impl Board {
    fn piece_at(&self, square: usize) -> Option<Piece> {
        let mask: Bitboard = 1 << square;
        Piece::ALL.into_iter().find(|&piece| (self[piece] & mask) != 0)
    }

    pub fn can_castle(&self, castling_flag: Castling) -> bool {
        match castling_flag {
            Castling::WhiteQueenside => (self.castling_rights & 0b10000000) != 0,
            Castling::WhiteKingside  => (self.castling_rights & 0b01000000) != 0,
            Castling::BlackQueenside => (self.castling_rights & 0b00100000) != 0,
            Castling::BlackKingside  => (self.castling_rights & 0b00010000) != 0
        }
    }

    pub fn clear(&mut self) {
        for i in 0..12 {
            self.bitboards[i] = 0u64;
        }
    }

    pub fn new() -> Self {
        Board { bitboards: STARTING_BOARD, to_move: Color::White, en_passant: 0u64, castling_rights: 0b11110000 }
    }

    pub fn occupancy(&self, color: Color) -> Bitboard {
        let start = match color {
            Color::White => 0,
            Color::Black => 6,
        };

        let mut occupancy = 0;

        for i in start..start + 6 {
            occupancy |= self.bitboards[i];
        }

        occupancy
    }

    pub fn print(&self) {
        for rank in (0..8).rev() {
            let mut row = format!("{}", rank + 1);
            for file in 0..8 {
                let square = rank * 8 + file;
                let symbol = match self.piece_at(square) {
                    Some(piece) => piece.to_char(),
                    None => '.'
                };
                row.push(' ');
                row.push(symbol);
                row.push(' ');
            }
            println!("{row}");
        }
        println!("  A  B  C  D  E  F  G  H  \n");
    }
}

impl Index<Piece> for Board {
    type Output = Bitboard;

    fn index(&self, piece: Piece) -> &Self::Output {
        &self.bitboards[piece.index()]
    }
}

impl IndexMut<Piece> for Board {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        &mut self.bitboards[piece.index()]
    }
}