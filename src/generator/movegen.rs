use crate::generator::{Bitboard, Board, Color, Move, Piece, PieceType, bitboard::print_bitboard};

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

// Functions to actually get the generated moves
pub struct Generator {
    b: Board,
    color_offset_idx: i32,
    self_occ: Bitboard,
    oppn_occ: Bitboard
}

impl Generator {
    pub fn new(b: Board) -> Self {
        let color_offset_idx = match b.to_move {
            Color::White => 0,
            Color::Black => 6
        };

        Generator { b: b, color_offset_idx: color_offset_idx, self_occ: 0u64, oppn_occ: 0u64 }
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

        self.get_knight_moves(&mut move_list);

        move_list
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



