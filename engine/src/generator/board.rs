use std::{ops::{Index, IndexMut}, println};
use crate::generator::{Bitboard, Castling, Color, Move, PieceType, bitboard::{BLACK_KS, BLACK_QS, CASTLING_RIGHTS_MASK, CASTLING_ROOK_XOR_DEST, STARTING_BOARD, WHITE_KS, WHITE_QS, north, south}};
use super::piece::Piece;

#[derive(Copy, Clone)]
pub struct Board {
    pub bitboards: [Bitboard; 12],
    pub to_move: Color,
    pub en_passant: Bitboard, // the square a pawn may move to on the next move completing an en-passant capture (empty if not available)
    pub castling_rights: u8, // [White QS][White KS][Black QS][Black KS][----]
    pub phase: f32,
}

pub struct UndoState {
    // the move that was made to achieve this state
    pub mv: Move,
    // The board as was before the last move was made
    pub to_move: Color,
    pub en_passant: Bitboard,
    pub castling_rights: u8,
    pub phase: f32,
}

const PHASE_WEIGHTS: [i32; 6] = [0, 1, 1, 2, 4, 0]; // P, N, B, R, Q, K
const TOTAL_PHASE: i32 = 24; // 4*N + 4*B + 4*R + 2*Q = 4 + 4 + 8 + 8

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn get_fen(&self) -> String {
        let mut fen = String::new();

        // 1) Piece Placement
        // Standard piece character mapping for indices 0..11:
        // White: P, N, B, R, Q, K (0-5)
        // Black: p, n, b, r, q, k (6-11)
        for rank in (0..8).rev() { // 8 --> 1
            let mut empty_count = 0;
            for file in 0..8 {
                let square = rank * 8 + file;
                let mut found_piece: Option<Piece> = None;

                // Search this square
                for (piece_idx, bb) in self.bitboards.iter().enumerate() {
                    if (bb & (1 << square)) != 0 {
                        found_piece = Some(Piece::new_from_index(piece_idx));
                        break;
                    }
                }

                if let Some(piece) = found_piece {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    fen.push(piece.to_char());
                } else {
                    empty_count += 1;
                }
            }

            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }

            if rank > 0 {
                fen.push('/');
            }
        }

        fen.push(' ');

        // 2. Active Color
        let color_char = if self.to_move == Color::White { 'w' } else { 'b' };
        fen.push(color_char);
        fen.push(' ');

        // 3. Castling Rights
        // Based on comment: [White QS][White KS][Black KS][Black QS][----] 
        // Adjust bit masks or helper methods depending on your exact u8 bit layout.
        let mut castling = String::new();
        if self.can_castle(Castling::WhiteKingside) { castling.push('K'); }
        if self.can_castle(Castling::WhiteQueenside) { castling.push('Q'); }
        if self.can_castle(Castling::BlackKingside) { castling.push('k'); }
        if self.can_castle(Castling::BlackQueenside) { castling.push('q'); }

        if castling.is_empty() {
            fen.push('-');
        } else {
            fen.push_str(&castling);
        }
        fen.push(' ');

        // 4. En Passant Target Square
        // Assuming self.en_passant is a Bitboard containing at most one bit.
        if self.en_passant > 0 {
            let sq = self.en_passant.trailing_zeros();
            let file = (sq % 8) as u8;
            let rank = (sq / 8) as u8;
            let file_char = (b'a' + file) as char;
            let rank_char = (b'1' + rank) as char;
            fen.push(file_char);
            fen.push(rank_char);
        } else {
            fen.push('-');
        }

        // 5 & 6. Halfmove clock and Fullmove number (Defaults)
        fen.push_str(" 0 1");

        fen
    }

    // Returns a value between 0.0 (opening) and 1.0 (endgame)
    pub fn get_phase(&self) -> f32 {
        // TODO: Store phase and just update in make/unmake saves recalculating every time
        let mut material = 0;
        for (piece, phase) in PHASE_WEIGHTS.iter().enumerate() {
            material += phase
                * (self.bitboards[piece].count_ones() + self.bitboards[6 + piece].count_ones()) as i32;
        }

        // Promotions can push material above the starting total
        let material = material.min(TOTAL_PHASE);

        // 0.0 = opening (all material), 1.0 = endgame (bare kings/pawns)
        (TOTAL_PHASE - material) as f32 / TOTAL_PHASE as f32
    }

    fn remove_phase_material(&mut self, piece: PieceType) {
        let material_curr = (TOTAL_PHASE as f32) - (self.phase * TOTAL_PHASE as f32);
        let mut material_new = material_curr - PHASE_WEIGHTS[piece as usize] as f32;
        material_new = material_new.min(TOTAL_PHASE as f32);
        self.phase = (TOTAL_PHASE as f32 - material_new) / (TOTAL_PHASE as f32);
    }

    fn add_phase_material(&mut self, piece: PieceType) {
        let material_curr = (TOTAL_PHASE as f32) - (self.phase * TOTAL_PHASE as f32);
        let mut material_new = material_curr + PHASE_WEIGHTS[piece as usize] as f32;
        material_new = material_new.min(TOTAL_PHASE as f32);
        self.phase = (TOTAL_PHASE as f32 - material_new) / (TOTAL_PHASE as f32);
    }

    pub fn unmake_move(&mut self, undo: UndoState) {
        // Put the piece that was moved back to its original square
        let piece = undo.mv.piece();
        let to = undo.mv.to();
        let (offset, inverse_offset) = match undo.to_move {
            Color::White => (0, 6),
            Color::Black => (6, 0),
        };

        // Whilst the below is nice, this breaks in the case of promotion
        // self.bitboards[piece as usize + offset] ^= undo.mv.from() | undo.mv.to(); // Clear the bit at to and set it at from

        // Place the piece that was moved back at from
        self.bitboards[piece as usize + offset] |= undo.mv.from();

        if let Some(captured_piece_type) = undo.mv.captured_piece() {
            if undo.mv.is_en_passant() {
                // Put the captured pawn back on the square "behind" the pawn that moved
                match undo.to_move {
                    Color::White => self.bitboards[Piece::BlackPawn as usize] |= south(to),
                    Color::Black => self.bitboards[Piece::WhitePawn as usize] |= north(to),
                }
                self.add_phase_material(PieceType::Pawn);
            } else {
                // get the piece type that was captured and put it back at to()
                self.bitboards[captured_piece_type as usize + inverse_offset] |= to;
                self.add_phase_material(captured_piece_type);
            }
        } else if let Some(castling_type) = undo.mv.castling_is() {
            // Put the corresponding rook back to its original square
            let rook_idx = PieceType::Rook as usize + offset;
            self.bitboards[rook_idx] ^= CASTLING_ROOK_XOR_DEST[castling_type as usize];
        }

        // If move is a promotion, it identifies the piece type that was promoted to and deletes it from the
        // corresponding bitboard, otherwise defaults to the piece that was moved and deletes it
        let piece_type = undo.mv.promotes_to().unwrap_or(piece);
        self.bitboards[piece_type as usize + offset] &= !to;

        if undo.mv.is_promotion() {
            self.add_phase_material(PieceType::Pawn);
            self.remove_phase_material(piece_type);
        }

        self.to_move = undo.to_move;
        self.en_passant = undo.en_passant;
        self.castling_rights = undo.castling_rights;
    }

    pub fn make_move(&mut self, mv: Move) -> UndoState {
        let undo = UndoState {
            mv,
            to_move: self.to_move,
            en_passant: self.en_passant,
            castling_rights: self.castling_rights,
            phase: self.phase
        };

        let dest = mv.to();

        let (bb_idx, color_offset, self_color_offset, ep_pawn_idx) = match self.to_move {
            Color::White => (mv.piece() as usize, 6, 0, Piece::BlackPawn as usize), 
            Color::Black => (mv.piece() as usize + 6, 0, 6, Piece::WhitePawn as usize)
        };

        // move the target piece from the origin to the target square
        self.bitboards[bb_idx] ^= mv.from() | dest; // Clear the bit at from and set it at to

        // Disable future en-passant, this is updated if the move was a double pawn push
        self.en_passant = 0;

        // If the move was a capturing move (includes en-passant) update the captured
        // pieces bitboard
        if mv.is_capture() {
            if mv.is_en_passant() { // Capturing an enemy pawn and we can easily work out where
                let captured_pawn = if self.to_move == Color::White {
                    south(dest)
                } else {
                    north(dest)
                };
                self.bitboards[ep_pawn_idx] &= !captured_pawn;
                
                // Update the phase to remove a pawn
                self.remove_phase_material(PieceType::Pawn);

            } else {
                let captured_piece = mv.captured_piece().unwrap();
                let captured_idx = captured_piece as usize + color_offset;
                self.remove_phase_material(captured_piece);
                self.bitboards[captured_idx] &= !dest;
            }
        }

        if mv.is_double_pawn_push() {
            self.en_passant = match self.to_move {
                Color::White => south(dest),
                Color::Black => north(dest)
            };
        }

        if let Some(promotion_piece_type) = mv.promotes_to() {
            let promotion_idx = promotion_piece_type as usize + self_color_offset;
            self.bitboards[promotion_idx] |= dest;

            // Delete pawn from mbv.to() (as already moved)
            self.bitboards[bb_idx] &= !dest;

            self.remove_phase_material(PieceType::Pawn);
            self.add_phase_material(promotion_piece_type);
        }

        // Castling (move the rook) - King has already been moved
        if let Some(c) = mv.castling_is() {
            // Either 0, 1, 2 or 3 so just index into the array of XOR masks to move the rook to its new square
            let rook_idx = PieceType::Rook as usize + self_color_offset;
            self.bitboards[rook_idx] ^= CASTLING_ROOK_XOR_DEST[c as usize];
        }

        // Update castling rights
        // 1. If a piece moves FROM E1, A1, H1, E8, A8, H8 -> updates castling rights
        // 2. If a piece is captured ON A1, H1, A8, H8 -> updates castling rights
        // 3. For any other square, ANDing 0b11110000 is a no-op that leaves rights intact
        self.castling_rights &= CASTLING_RIGHTS_MASK[mv.from_index() as usize];
        self.castling_rights &= CASTLING_RIGHTS_MASK[mv.to_index() as usize];

        // switch to_move over to the other player
        self.to_move = !self.to_move;

        // return the undo state
        undo
    }

    fn piece_at(&self, square: usize) -> Option<Piece> {
        let mask: Bitboard = 1 << square;
        Piece::ALL.into_iter().find(|&piece| (self[piece] & mask) != 0)
    }

    pub fn can_castle(&self, castling_flag: Castling) -> bool {
        match castling_flag {
            Castling::WhiteQueenside => (self.castling_rights & WHITE_QS) != 0,
            Castling::WhiteKingside  => (self.castling_rights & WHITE_KS) != 0,
            Castling::BlackQueenside => (self.castling_rights & BLACK_QS) != 0,
            Castling::BlackKingside  => (self.castling_rights & BLACK_KS) != 0
        }
    }

    pub fn clear(&mut self) {
        for i in 0..12 {
            self.bitboards[i] = 0u64;
        }
    }

    pub fn new() -> Self {
        Board { 
            bitboards: STARTING_BOARD, 
            to_move: Color::White, 
            en_passant: 0u64, 
            castling_rights: 0b11110000,
            phase: 0.0
        }
    }

    pub fn from_fen(fen: &str) -> Option<Self> {
        let mut parts = fen.split_whitespace();
        let placement = parts.next()?;
        let side = parts.next().unwrap_or("w");
        let castling = parts.next().unwrap_or("-");
        let ep = parts.next().unwrap_or("-");

        let mut bbs: [Bitboard; 12] = [0; 12];
        let mut rank: i32 = 7;
        let mut file: i32 = 0;

        for c in placement.chars() {
            if c == '/' {
                rank -= 1;
                file = 0;
                continue;
            }
            if let Some(n) = c.to_digit(10) {
                file += n as i32;
                continue;
            }
            let piece = match c {
                'p' => Piece::BlackPawn,   'n' => Piece::BlackKnight,
                'b' => Piece::BlackBishop, 'r' => Piece::BlackRook,
                'q' => Piece::BlackQueen,  'k' => Piece::BlackKing,
                'P' => Piece::WhitePawn,   'N' => Piece::WhiteKnight,
                'B' => Piece::WhiteBishop, 'R' => Piece::WhiteRook,
                'Q' => Piece::WhiteQueen,  'K' => Piece::WhiteKing,
                _ => return None,
            };
            if !(0..8).contains(&rank) || !(0..8).contains(&file) {
                return None;
            }
            let sq = (rank * 8 + file) as u32;
            bbs[piece as usize] |= 1u64 << sq;
            file += 1; // <-- the missing line
        }

        let to_move = match side {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return None,
        };

        // Adjust these bits to match however your engine encodes castling rights
        let mut castling_rights = 0u8;
        for c in castling.chars() {
            match c { // [White QS][White KS][Black QS][Black KS][----]
                'K' => castling_rights |= WHITE_KS,
                'Q' => castling_rights |= WHITE_QS,
                'k' => castling_rights |= BLACK_KS,
                'q' => castling_rights |= BLACK_QS,
                _ => {}
            }
        }

        let en_passant = if ep == "-" {
            0u64
        } else {
            let b = ep.as_bytes();
            if b.len() != 2 { return None; }
            let f = b[0].checked_sub(b'a')? as u32;
            let r = b[1].checked_sub(b'1')? as u32;
            if f > 7 || r > 7 { return None; }
            1u64 << (r * 8 + f)
        };

        let mut b = Board { bitboards: bbs, to_move, en_passant, castling_rights, phase: 0.0 };
        b.phase = b.get_phase();

        Some(b)
    }

    pub fn total_occupancy(&self) -> Bitboard {
        let mut occupancy = 0;
        for bb in &self.bitboards {
            occupancy |= *bb;
        }
        occupancy
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
