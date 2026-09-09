use std::ops::{Index, IndexMut};
use super::piece::Piece;

pub struct Board {
    bitboards: [u64; 12]
}

impl Board {
    fn piece_at(&self, square: usize) -> Option<Piece> {
        let mask = 1u64 << square;
        Piece::ALL.into_iter().find(|&piece| (self[piece] & mask) != 0)
    }

    pub fn new() -> Self {
        Board { bitboards: [0; 12] }
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
        println!("  A  B  C  D  E  F  G  H  ");
    }
}

impl Index<Piece> for Board {
    type Output = u64;

    fn index(&self, piece: Piece) -> &Self::Output {
        &self.bitboards[piece.index()]
    }
}

impl IndexMut<Piece> for Board {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        &mut self.bitboards[piece.index()]
    }
}