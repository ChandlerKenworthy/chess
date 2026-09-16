use std::{debug_assert, ops::{Index, IndexMut}};
use crate::generator::{Bitboard, Color, bitboard::STARTING_BOARD};
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

    pub fn from(self) -> Bitboard {
        1 << ((self.0 >> Self::FROM_SHIFT) & Self::SQUARE_MASK)
    }

    pub fn from_index(self) -> u32 {
        (self.0 >> Self::FROM_SHIFT) & Self::SQUARE_MASK
    }
    
    pub fn to(self) -> Bitboard {
        1 << ((self.0 >> Self::TO_SHIFT) & Self::SQUARE_MASK)
    }

    pub fn to_index(self) -> u32 {
        (self.0 >> Self::TO_SHIFT) & Self::SQUARE_MASK
    }

}

#[derive(Copy, Clone, Debug)]
pub struct Square(u8);

impl Square {
    pub fn new(index: u32) -> Self {
        debug_assert!(index < 64);
        Self(index as u8)
    }

    pub fn index(self) -> u8 {
        self.0
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
    pub to_move: Color
}

impl Board {
    fn piece_at(&self, square: usize) -> Option<Piece> {
        let mask: Bitboard = 1 << square;
        Piece::ALL.into_iter().find(|&piece| (self[piece] & mask) != 0)
    }

    pub fn new() -> Self {
        Board { bitboards: STARTING_BOARD, to_move: Color::White }
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
            let mut row = format!("{rank}");
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