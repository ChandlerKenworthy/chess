use std::println;

use crate::generator::{Bitboard, Board, Color, Move, Piece, PieceType, bitboard::{PRIMARY_STRAIGHT_RAYS, SECONDARY_STRAIGHT_RAYS, PRIMARY_DIAGONAL_RAYS, SECONDARY_DIAGONAL_RAYS}};

// Functions to generate lookup tables
const fn generate_knight_moves(square: usize) -> Bitboard {
    let rank = square / 8;
    let file = square % 8;

    let mut moves = 0u64;

    const KNIGHT_OFFSETS: [(isize, isize); 8] = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];

    let mut i = 0;
    while i < KNIGHT_OFFSETS.len() {
        let new_rank = rank as isize + KNIGHT_OFFSETS[i].0;
        let new_file = file as isize + KNIGHT_OFFSETS[i].1;

        if new_rank >= 0
            && new_rank < 8
            && new_file >= 0
            && new_file < 8
        {
            let target = (new_rank * 8 + new_file) as usize;
            moves |= 1u64 << target;
        }

        i += 1;
    }

    moves
}

// Lookup tables
pub const KNIGHT_MOVES: [Bitboard; 64] = {
    let mut table = [0; 64];

    let mut square = 0;
    while square < 64 {
        table[square] = generate_knight_moves(square);
        square += 1;
    }

    table
};

// Utility functions for move generation
fn hyperbola_quintessence(
    occupancy: Bitboard, 
    slider: Bitboard, 
    ray: Bitboard
) -> Bitboard {
    // General HQ formulation:
    // attacks = ((o - 2r) ^ reverse(reverse(o) - 2*reverse(r))) & ray
    // where o = occupancy & ray, r = slider
    //
    // Note: this relies on wrapping (two's complement) arithmetic by design,
    // so we must use wrapping_sub/wrapping_mul instead of -/* to avoid
    // Rust's debug-mode overflow panics.

    let occupied = occupancy & ray;
    let rev_occupied = occupied.reverse_bits();
    let rev_slider = slider.reverse_bits();

    let forward = occupied.wrapping_sub(slider.wrapping_mul(2));
    let backward = rev_occupied.wrapping_sub(rev_slider.wrapping_mul(2));

    (forward ^ backward.reverse_bits()) & ray
}

// Functions to actually get the generated moves
pub struct Generator {
    b: Board,
    color_offset_idx: i32,
    self_occ: Bitboard,
    oppn_occ: Bitboard,
    total_occ: Bitboard
}

impl Generator {
    pub fn new(b: Board) -> Self {
        let color_offset_idx = match b.to_move {
            Color::White => 0,
            Color::Black => 6
        };

        Generator { b: b, color_offset_idx: color_offset_idx, self_occ: 0u64, oppn_occ: 0u64, total_occ: 0u64 }
    }

    pub fn get_moves(&mut self) -> Vec<Move> {
        let mut move_list = Vec::new();

        // Find which colour is due to move for bb offsets
        self.color_offset_idx = match self.b.to_move {
            Color::White => 0,
            Color::Black => 6
        };

        // Calculate the current board occupancies
        self.self_occ = self.b.occupancy(self.b.to_move);
        self.oppn_occ = self.b.occupancy(!self.b.to_move);
        self.total_occ = self.self_occ | self.oppn_occ;

        self.get_knight_moves(&mut move_list);
        self.get_rook_moves(&mut move_list);
        self.get_bishop_moves(&mut move_list);

        move_list
    }

    // Use HQ for generating the sliding piece attacks
    // for now no lookup table
    fn get_rook_moves(&self, moves: &mut Vec<Move>) {
        let mut rooks = self.b.bitboards[(PieceType::Rook as usize) + self.color_offset_idx as usize];

        while rooks != 0 {
            let from_idx = rooks.trailing_zeros() as usize;
            let from: Bitboard = 1u64 << from_idx;

            let primary_ray = PRIMARY_STRAIGHT_RAYS[from_idx];
            let secondary_ray = SECONDARY_STRAIGHT_RAYS[from_idx];

            // Inefficient, as slider.reverse_bits() called each time for primary & secondary
            // despite being the same calculation
            let primary_attacks = hyperbola_quintessence(
                self.total_occ, from, primary_ray
            );
            let secondary_attacks = hyperbola_quintessence(
                self.total_occ, from, secondary_ray
            );

            let mut mvs = (primary_attacks | secondary_attacks) & !self.self_occ;
            while mvs != 0 {
                let to: u64 = 1 << mvs.trailing_zeros();

                // Create and store the move
                moves.push(Move::new(
                    from,
                    to,
                ));

                // Clear the least significant destination bit
                mvs &= mvs - 1;
            }
            rooks &= rooks - 1; // Remove the rook that was just processed
        }
    }

    fn get_bishop_moves(&self, moves: &mut Vec<Move>) {
        let mut bishops = self.b.bitboards[(PieceType::Bishop as usize) + self.color_offset_idx as usize];

        while bishops != 0 {
            let from_idx = bishops.trailing_zeros() as usize;
            let from: Bitboard = 1u64 << from_idx;

            let primary_ray = PRIMARY_DIAGONAL_RAYS[from_idx];
            let secondary_ray = SECONDARY_DIAGONAL_RAYS[from_idx];

            let primary_attacks = hyperbola_quintessence(
                self.total_occ, from, primary_ray
            );
            let secondary_attacks = hyperbola_quintessence(
                self.total_occ, from, secondary_ray
            );

            let mut mvs = (primary_attacks | secondary_attacks) & !self.self_occ;
            while mvs != 0 {
                let to: u64 = 1 << mvs.trailing_zeros();

                // Create and store the move
                moves.push(Move::new(
                    from,
                    to,
                ));
                mvs &= mvs - 1;
            }
            bishops &= bishops - 1; // Remove the rook that was just processed
        }
    }


    fn get_knight_moves(&self, moves: &mut Vec<Move>) {
        // for now just deal with the knights
        let mut knights = self.b.bitboards[(PieceType::Knight as usize) + self.color_offset_idx as usize];

        while knights != 0 {
            // Get index of the least significant knight
            let from_idx = knights.trailing_zeros() as usize;
            let from: u64 = 1 << from_idx;

            // Get all possible knight destinations
            let mut mvs = KNIGHT_MOVES[from_idx];

            // Can't move onto one of our own pieces
            mvs &= !self.self_occ;

            // Iterate through every destination square
            while mvs != 0 {
                let to: u64 = 1 << mvs.trailing_zeros();

                // Create and store the move
                moves.push(Move::new(
                    from,
                    to,
                ));

                // Clear the least significant destination bit
                mvs &= mvs - 1;
            }

            // Clear the knight we just processed
            knights &= knights - 1;
        }
    }
}



