use crate::generator::{Bitboard, Board, Color, Move, Piece, PieceType, bitboard::{
    BLACK_KINGSIDE_ATTACKS_KING_OR_PAWN, BLACK_KINGSIDE_ATTACKS_KNIGHT, BLACK_KINGSIDE_FREE_SQUARES, BLACK_KINGSIDE_SLIDER_ATTACKS, BLACK_QUEENSIDE_ATTACKS_KING_OR_PAWN, BLACK_QUEENSIDE_ATTACKS_KNIGHT, BLACK_QUEENSIDE_FREE_SQUARES, BLACK_QUEENSIDE_SLIDER_ATTACKS, KINGSIDE_CASTLE_KING_DEST, PRIMARY_DIAGONAL_RAYS, PRIMARY_STRAIGHT_RAYS, PROMOTE_BISHOP_AND_CAPTURE, PROMOTE_KNIGHT_AND_CAPTURE, PROMOTE_QUEEN_AND_CAPTURE, PROMOTE_ROOK_AND_CAPTURE, QUEENSIDE_CASTLE_KING_DEST, RANK_1, RANK_3, RANK_6, RANK_8, REVERSE_SQUARE_BB, SECONDARY_DIAGONAL_RAYS, SECONDARY_STRAIGHT_RAYS, WHITE_KINGSIDE_ATTACKS_KING_OR_PAWN, WHITE_KINGSIDE_ATTACKS_KNIGHT, WHITE_KINGSIDE_FREE_SQUARES, WHITE_KINGSIDE_SLIDER_ATTACKS, WHITE_QUEENSIDE_ATTACKS_KING_OR_PAWN, WHITE_QUEENSIDE_ATTACKS_KNIGHT, WHITE_QUEENSIDE_FREE_SQUARES, WHITE_QUEENSIDE_SLIDER_ATTACKS, east, north, south, west
}, piece::Castling};
use arrayvec::ArrayVec;

pub type MoveList = ArrayVec<Move, 256>;

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

const fn generate_king_moves(square: usize) -> Bitboard {
    let from: Bitboard = 1 << square;

    north(from) | south(from) | east(from) | west(from) 
    | north(east(from)) | north(west(from)) | south(east(from)) | south(west(from))
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

pub const KING_MOVES: [Bitboard; 64] = {
    let mut table = [0; 64];

    let mut square = 0;
    while square < 64 {
        table[square] = generate_king_moves(square);
        square += 1;
    }

    table
};

// Utility functions for move generation
#[inline(always)]
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

    let forward = occupied.wrapping_sub(slider.wrapping_mul(2));
    let backward = rev_occupied.wrapping_sub(REVERSE_SQUARE_BB[slider.trailing_zeros() as usize].wrapping_mul(2));

    (forward ^ backward.reverse_bits()) & ray
}

// Functions to actually get the generated moves
pub struct Generator {
    pub b: Board,
    color_offset_idx: i32,
    oppn_color_offset_idx: i32,
    self_occ: Bitboard,
    not_self_occ: Bitboard,
    oppn_occ: Bitboard,
    total_occ: Bitboard
}

impl Generator {
    pub fn new(b: Board) -> Self {
        let color_offset_idx = match b.to_move {
            Color::White => 0,
            Color::Black => 6
        };

        let oppn_color_offset_idx = match b.to_move {
            Color::White => 6,
            Color::Black => 0
        };

        Generator { b, color_offset_idx, oppn_color_offset_idx, self_occ: 0u64, not_self_occ: !0u64, oppn_occ: 0u64, total_occ: 0u64 }
    }

    fn is_in_check(&self, color: Color) -> bool {
        let (king, offset, pawn_attack) = match color {
            Color::White => (
                self.b.bitboards[Piece::WhiteKing as usize],
                6,
                north as fn(u64) -> u64,
            ),
            Color::Black => (
                self.b.bitboards[Piece::BlackKing as usize],
                0,
                south as fn(u64) -> u64,
            ),
        };

        let king_idx = king.trailing_zeros() as usize;
        
        if KNIGHT_MOVES[king_idx]
        & self.b.bitboards[PieceType::Knight as usize + offset] != 0 {
            return true;
        }

        let checking_enemy_pawns = pawn_attack(east(king) | west(king));
        if (checking_enemy_pawns & self.b.bitboards[PieceType::Pawn as usize + offset]) != 0 {
            return true;
        }

        if KING_MOVES[king_idx] & self.b.bitboards[PieceType::King as usize + offset] != 0 {
            return true;
        }

        let occupancy = self.b.total_occupancy();

        let enemy_queen = self.b.bitboards[(PieceType::Queen as usize) + offset];
        let enemy_straight_sliders = self.b.bitboards[(PieceType::Rook as usize) + offset] | enemy_queen;
        let enemy_diagonal_sliders = self.b.bitboards[(PieceType::Bishop as usize) + offset] | enemy_queen;

        let ray_attacks = hyperbola_quintessence(occupancy, king, PRIMARY_STRAIGHT_RAYS[king_idx]);
        if (ray_attacks & enemy_straight_sliders) != 0 {
            return true;
        }

        let ray_attacks = hyperbola_quintessence(occupancy, king, SECONDARY_STRAIGHT_RAYS[king_idx]);
        if (ray_attacks & enemy_straight_sliders) != 0 {
            return true;
        }

        let ray_attacks = hyperbola_quintessence(occupancy, king, PRIMARY_DIAGONAL_RAYS[king_idx]);
        if (ray_attacks & enemy_diagonal_sliders) != 0 {
            return true;
        }

        let ray_attacks = hyperbola_quintessence(occupancy, king, SECONDARY_DIAGONAL_RAYS[king_idx]);
        if (ray_attacks & enemy_diagonal_sliders) != 0 {
            return true;
        }

        false
    }

    fn get_pseudo_legal_moves(&mut self) -> MoveList {
        let mut move_list = MoveList::new();

        // Find which colour is due to move for bb offsets
        (self.color_offset_idx, self.oppn_color_offset_idx) = match self.b.to_move {
            Color::White => (0, 6),
            Color::Black => (6, 0)
        };

        // Calculate the current board occupancies
        self.self_occ = self.b.occupancy(self.b.to_move);
        self.not_self_occ = !self.self_occ; // this operation is performed a lot so reduces # of instructions by doing it once here
        self.oppn_occ = self.b.occupancy(!self.b.to_move);
        self.total_occ = self.self_occ | self.oppn_occ;

        // pawn moves single, double on first (if both clear), en-passant
        // king moves do not incl. castling yet
        self.get_king_moves(&mut move_list);
        self.get_castling_moves(&mut move_list);
        self.get_knight_moves(&mut move_list);
        self.get_rook_moves(&mut move_list);
        self.get_bishop_moves(&mut move_list);
        self.get_queen_moves(&mut move_list);
        self.get_pawn_moves(&mut move_list);

        move_list
    }

    /// Returns a list of all legal moves for the current position.
    pub fn get_moves(&mut self) -> MoveList {
        let mut legal_moves = MoveList::new();
        let pseudo_legal = self.get_pseudo_legal_moves(); // ideally also returns MoveList!

        for mv in pseudo_legal {
            let undo = self.b.make_move(mv);
            let is_legal = !self.is_in_check(!self.b.to_move);
            self.b.unmake_move(undo);

            if is_legal {
                legal_moves.push(mv); // Stack push (zero allocations)
            }
        }

        legal_moves
    }

    #[inline(always)]
    fn add_moves(&self, moves: &mut MoveList, mut mvs: Bitboard, from: Bitboard, p: PieceType) {
        while mvs != 0 {
            let to: u64 = 1 << mvs.trailing_zeros();
            moves.push(Move::new(
                from,
                to,
                p
            ));
            mvs &= mvs - 1;
        }
    }

    #[inline(always)]
    fn add_capture_moves(&self, moves: &mut MoveList, mut mvs: Bitboard, from: Bitboard, p: PieceType) {
        while mvs != 0 {
            let to: u64 = 1 << mvs.trailing_zeros();
            let captured = self.get_captured_piece(to).unwrap();
            moves.push(Move::new_capture(
                from,
                to,
                p,
                captured
            ));
            mvs &= mvs - 1;
        }
    }

    #[inline(always)]
    fn add_moves_with_flags(&self, moves: &mut MoveList, mut mvs: Bitboard, from: Bitboard, p: PieceType, flags: u32) {
        while mvs != 0 {
            let to: u64 = 1 << mvs.trailing_zeros();
            moves.push(Move::new_with_flags(
                from,
                to,
                p,
                flags
            ));
            mvs &= mvs - 1;
        }
    }

    #[inline(always)]
    fn get_captured_piece(&self, target_mask: Bitboard) -> Option<PieceType> {
        let opp_offset = self.oppn_color_offset_idx as usize;
        let bbs = &self.b.bitboards;

        if (bbs[PieceType::Pawn as usize + opp_offset] & target_mask) != 0 {
            return Some(PieceType::Pawn);
        } else if (bbs[PieceType::Knight as usize + opp_offset] & target_mask) != 0 {
            return Some(PieceType::Knight);
        } else if (bbs[PieceType::Bishop as usize + opp_offset] & target_mask) != 0 {
            return Some(PieceType::Bishop);
        } else if (bbs[PieceType::Rook as usize + opp_offset] & target_mask) != 0 {
            return Some(PieceType::Rook);
        } else if (bbs[PieceType::Queen as usize + opp_offset] & target_mask) != 0 {
            return Some(PieceType::Queen);
        }

        None
    }

    // Use HQ for generating the sliding piece attacks
    // for now no lookup table
    fn get_rook_moves(&self, moves: &mut MoveList) {
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

            let mvs = (primary_attacks | secondary_attacks) & self.not_self_occ;
            let attack_moves = mvs & self.oppn_occ;
            let other_moves= mvs & !attack_moves;

            self.add_capture_moves(moves, attack_moves, from, PieceType::Rook);
            self.add_moves(moves, other_moves, from, PieceType::Rook);

            rooks &= rooks - 1; // Remove the rook that was just processed
        }
    }

    fn get_bishop_moves(&self, moves: &mut MoveList) {
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

            let mvs = (primary_attacks | secondary_attacks) & self.not_self_occ;
            let attack_moves = mvs & self.oppn_occ;
            let other_moves= mvs & !attack_moves;

            self.add_capture_moves(moves, attack_moves, from, PieceType::Bishop);
            self.add_moves(moves, other_moves, from, PieceType::Bishop);

            bishops &= bishops - 1;
        }
    }

    fn get_queen_moves(&self, moves: &mut MoveList) {
        let mut queens = self.b.bitboards[(PieceType::Queen as usize) + self.color_offset_idx as usize];

        while queens != 0 {
            let from_idx = queens.trailing_zeros() as usize;
            let from: Bitboard = 1u64 << from_idx;

            let primary_diagonal_ray = PRIMARY_DIAGONAL_RAYS[from_idx];
            let secondary_diagonal_ray = SECONDARY_DIAGONAL_RAYS[from_idx];
            let primary_straight_ray = PRIMARY_STRAIGHT_RAYS[from_idx];
            let secondary_straight_ray = SECONDARY_STRAIGHT_RAYS[from_idx];

            let primary_diagonal_attacks = hyperbola_quintessence(
                self.total_occ, from, primary_diagonal_ray
            );
            let secondary_diagonal_attacks = hyperbola_quintessence(
                self.total_occ, from, secondary_diagonal_ray
            );
            let primary_straight_attacks = hyperbola_quintessence(
                self.total_occ, from, primary_straight_ray
            );
            let secondary_straight_attacks = hyperbola_quintessence(
                self.total_occ, from, secondary_straight_ray
            );

            let mvs = (
                primary_straight_attacks 
                | secondary_straight_attacks 
                | primary_diagonal_attacks 
                | secondary_diagonal_attacks) & self.not_self_occ;
            let attack_moves = mvs & self.oppn_occ;
            let other_moves= mvs & !attack_moves;

            self.add_capture_moves(moves, attack_moves, from, PieceType::Queen);
            self.add_moves(moves, other_moves, from, PieceType::Queen);

            queens &= queens - 1;
        }
    }

    fn get_knight_moves(&self, moves: &mut MoveList) {
        // for now just deal with the knights
        let mut knights = self.b.bitboards[(PieceType::Knight as usize) + self.color_offset_idx as usize];

        while knights != 0 {
            // Get index of the least significant knight
            let from_idx = knights.trailing_zeros() as usize;
            let from: u64 = 1 << from_idx;

            // Get all possible knight destinations - but not onto our own pieces
            let mvs = KNIGHT_MOVES[from_idx] & self.not_self_occ;
            let attack_moves = mvs & self.oppn_occ;
            let other_moves= mvs & !attack_moves;

            self.add_capture_moves(moves, attack_moves, from, PieceType::Knight);
            self.add_moves(moves, other_moves, from, PieceType::Knight);

            // Clear the knight we just processed
            knights &= knights - 1;
        }
    }

    fn get_king_moves(&self, moves: &mut MoveList) {
        let king = self.b.bitboards[(PieceType::King as usize) + self.color_offset_idx as usize];
        let from_idx = king.trailing_zeros() as usize;
        let from: u64 = 1 << from_idx;

        let mvs = KING_MOVES[from_idx] & self.not_self_occ;
        let attack_moves = mvs & self.oppn_occ;
        let other_moves= mvs & !attack_moves;

        self.add_capture_moves(moves, attack_moves, from, PieceType::King);
        self.add_moves(moves, other_moves, from, PieceType::King);
    }

    fn get_castling_moves(&self, moves: &mut MoveList) {
        let king = self.b.bitboards[(PieceType::King as usize) + self.color_offset_idx as usize];

        match self.b.to_move {
            Color::White => {
                if self.can_castle_kingside(Color::White) {
                    moves.push(Move::new_with_flags(king, KINGSIDE_CASTLE_KING_DEST[0], PieceType::King, 1 << 19));
                }
                if self.can_castle_queenside(Color::White) {
                    moves.push(Move::new_with_flags(king, QUEENSIDE_CASTLE_KING_DEST[0], PieceType::King, 1 << 18));
                }
            }
            Color::Black => {
                if self.can_castle_kingside(Color::Black) {
                    moves.push(Move::new_with_flags(king, KINGSIDE_CASTLE_KING_DEST[1], PieceType::King, 1 << 21));
                }
                if self.can_castle_queenside(Color::Black) {
                    moves.push(Move::new_with_flags(king, QUEENSIDE_CASTLE_KING_DEST[1], PieceType::King, 1 << 20));
                }
            }
        }
    }

    fn can_castle(&self, 
        rights: Castling, 
        free_squares: Bitboard, 
        king_pawn_mask: Bitboard, 
        knight_mask: Bitboard, 
        mut slider_mask: Bitboard, 
        offset: usize
    ) -> bool {
        // Check the king or rook have not moved before
        if !self.b.can_castle(rights) {
            return false;
        }
        // Check all the squares along the rank that need to be free are free
        if (self.total_occ & free_squares) != 0 {
            return false;
        }

        let enemy_king_or_pawn = self.b.bitboards[PieceType::King as usize + offset] | self.b.bitboards[PieceType::Pawn as usize + offset];
        // An enemy king or pawns attack "rays" intercept the castling squares the king will move through/onto
        if (king_pawn_mask & enemy_king_or_pawn) != 0 {
            return false;
        }

        if (knight_mask & self.b.bitboards[PieceType::Knight as usize + offset]) != 0 {
            return false;
        }

        // Iterate over the squares no attackers sliding piece should be able
        // to attack i.e. HQ from the square must be empty
        let enemy_queens = self.b.bitboards[PieceType::Queen as usize + offset];
        let enemy_rooks = self.b.bitboards[PieceType::Rook as usize + offset];
        let enemy_bishops = self.b.bitboards[PieceType::Bishop as usize + offset];

        while slider_mask != 0 {
            // this also prevents castling when in check as the king's square (e1/8)
            // is included in the slider mask
            let from_idx = slider_mask.trailing_zeros() as usize;
            let from: Bitboard = 1 << from_idx;
            
            let primary_rays = hyperbola_quintessence(self.total_occ, from, PRIMARY_STRAIGHT_RAYS[from_idx]);
            let secondary_rays = hyperbola_quintessence(self.total_occ, from, SECONDARY_STRAIGHT_RAYS[from_idx]);

            if ((primary_rays | secondary_rays) & (enemy_queens | enemy_rooks)) != 0 {
                return false;
            }

            let primary_rays = hyperbola_quintessence(self.total_occ, from, PRIMARY_DIAGONAL_RAYS[from_idx]);
            let secondary_rays = hyperbola_quintessence(self.total_occ, from, SECONDARY_DIAGONAL_RAYS[from_idx]);

            if ((primary_rays | secondary_rays) & (enemy_queens | enemy_bishops)) != 0 {
                return false;
            }

            slider_mask &= slider_mask - 1;
        }

        true
    }

    fn can_castle_kingside(&self, color: Color) -> bool {
        let (rights, free_squares, king_pawn_mask, knight_mask, slider_mask, offset) = match color {
            Color::White => (
                Castling::WhiteKingside,
                WHITE_KINGSIDE_FREE_SQUARES,
                WHITE_KINGSIDE_ATTACKS_KING_OR_PAWN,
                WHITE_KINGSIDE_ATTACKS_KNIGHT,
                WHITE_KINGSIDE_SLIDER_ATTACKS,
                6,
            ),
            Color::Black => (
                Castling::BlackKingside,
                BLACK_KINGSIDE_FREE_SQUARES,
                BLACK_KINGSIDE_ATTACKS_KING_OR_PAWN,
                BLACK_KINGSIDE_ATTACKS_KNIGHT,
                BLACK_KINGSIDE_SLIDER_ATTACKS,
                0,
            )
        };

        self.can_castle(rights, free_squares, king_pawn_mask, knight_mask, slider_mask, offset)
    }

    fn can_castle_queenside(&self, color: Color) -> bool {
        let (rights, free_squares, king_pawn_mask, knight_mask, slider_mask, offset) = match color {
            Color::White => (
                Castling::WhiteQueenside,
                WHITE_QUEENSIDE_FREE_SQUARES,
                WHITE_QUEENSIDE_ATTACKS_KING_OR_PAWN,
                WHITE_QUEENSIDE_ATTACKS_KNIGHT,
                WHITE_QUEENSIDE_SLIDER_ATTACKS,
                6,
            ),
            Color::Black => (
                Castling::BlackQueenside,
                BLACK_QUEENSIDE_FREE_SQUARES,
                BLACK_QUEENSIDE_ATTACKS_KING_OR_PAWN,
                BLACK_QUEENSIDE_ATTACKS_KNIGHT,
                BLACK_QUEENSIDE_SLIDER_ATTACKS,
                0,
            )
        };

        self.can_castle(rights, free_squares, king_pawn_mask, knight_mask, slider_mask, offset)
    }

    fn get_pawn_moves(&self, moves: &mut MoveList) {
        let mut pawns = self.b.bitboards[(PieceType::Pawn as usize) + self.color_offset_idx as usize];

        while pawns != 0 {
            let from_idx = pawns.trailing_zeros() as usize;
            let from: Bitboard = 1 << from_idx;

            // Compute the 1-step moves forward ("primary" pawn move)
            let mvs: Bitboard = match self.b.to_move {
                Color::White => north(from) & !self.total_occ,
                Color::Black => south(from) & !self.total_occ
            };
            let mut promotion_mvs = match self.b.to_move {
                Color::White => mvs & RANK_8,
                Color::Black => mvs & RANK_1
            };
            self.add_moves(moves, mvs & !promotion_mvs, from, PieceType::Pawn);
            
            while promotion_mvs != 0 {
                let to = 1 << promotion_mvs.trailing_zeros();
                
                moves.push( // promote to a queen
                    Move::new_with_flags(from, to, PieceType::Pawn, 1 << 22)
                );
                moves.push( // promote to a rook
                    Move::new_with_flags(from, to, PieceType::Pawn, 1 << 23)
                );
                moves.push( // promote to a knight
                    Move::new_with_flags(from, to, PieceType::Pawn, 1 << 24)
                );
                moves.push( // promote to a bishop
                    Move::new_with_flags(from, to, PieceType::Pawn, 1 << 25)
                );
                
                promotion_mvs &= promotion_mvs - 1;
            }

            // Now mvs has the 1-step forward we can compute double-step-forward moves
            // in a clever single-pass
            let mvs = match self.b.to_move {
                Color::White => north(mvs & RANK_3) & !self.total_occ,
                Color::Black => south(mvs & RANK_6) & !self.total_occ
            };
            self.add_moves_with_flags(moves, mvs, from, PieceType::Pawn, 0x10000);

            // Now scan for diagonal attacks
            let mvs = match self.b.to_move {
                Color::White => (north(east(from)) | north(west(from))) & self.oppn_occ,
                Color::Black => (south(east(from)) | south(west(from))) & self.oppn_occ
            };
            //print!("Diagonal pawn attacks!\n");
            //print_bitboard(mvs);
            // Flags are awkward here as could change e.g. all attacks, but could also
            // be a promotion!
            let mut attack_mv_promotion = match self.b.to_move {
                Color::White => mvs & RANK_8,
                Color::Black => mvs & RANK_1
            };
            let attack_mv = mvs & !attack_mv_promotion;
            self.add_capture_moves(moves, attack_mv, from, PieceType::Pawn);

            while attack_mv_promotion != 0 {
                let to = 1 << attack_mv_promotion.trailing_zeros();
                let captured = self.get_captured_piece(to).unwrap();

                moves.push( // Attack & promote to a queen
                    Move::new_capture_with_flags(from, to, PieceType::Pawn, captured, PROMOTE_QUEEN_AND_CAPTURE)
                );
                moves.push( // Attack & promote to a rook
                    Move::new_capture_with_flags(from, to, PieceType::Pawn, captured, PROMOTE_ROOK_AND_CAPTURE)
                );
                moves.push( // Attack & promote to a knight
                    Move::new_capture_with_flags(from, to, PieceType::Pawn, captured, PROMOTE_KNIGHT_AND_CAPTURE)
                );
                moves.push( // Attack & promote to a bishop
                    Move::new_capture_with_flags(from, to, PieceType::Pawn, captured, PROMOTE_BISHOP_AND_CAPTURE)
                );
                
                attack_mv_promotion &= attack_mv_promotion - 1;
            }
    
            if self.b.en_passant != 0 {
                let is_valid_ep = match self.b.to_move {
                    // No rank check needed of the origin pawn because self.b.en_passant is guaranteed to be on Rank 3/6
                    Color::White => (south(east(self.b.en_passant) | west(self.b.en_passant)) & from) != 0,
                    Color::Black => (north(east(self.b.en_passant) | west(self.b.en_passant)) & from) != 0,
                };

                if is_valid_ep {
                    moves.push(
                        Move::new_with_flags(
                            from, 
                            self.b.en_passant, 
                            PieceType::Pawn, 
                            0x28000
                        ));
                }
            }

            pawns &= pawns - 1;
        }
    }


}



 