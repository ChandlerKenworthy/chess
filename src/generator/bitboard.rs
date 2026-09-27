use crate::generator::Square;

pub type Bitboard = u64; // Bitboards are unsigned 64-bit integers

pub fn print_bitboard(bb: Bitboard) {
    for rank in (0..8).rev() {
        print!("{} ", rank + 1);
        for file in 0..8 {
            let square = rank * 8 + file;
            let occupied = (bb >> square) & 1;
            if occupied == 1 {
                print!(" X ");
            } else {
                print!(" . ");
            }
        }
        println!();
    }
    println!("   A  B  C  D  E  F  G  H  \n");
}

// Bit shift utilities with automatic wrapping avoidance
#[inline(always)]
pub const fn north(bb: Bitboard) -> Bitboard {
    bb << 8
}

#[inline(always)]
pub const fn south(bb: Bitboard) -> Bitboard {
    bb >> 8
}

#[inline(always)]
pub const fn east(bb: Bitboard) -> Bitboard {
    (bb << 1) & !FILE_A
}

#[inline(always)]
pub const fn west(bb: Bitboard) -> Bitboard {
    (bb >> 1) & !FILE_H
}

// Rank and file definitions
pub const RANK_1: Bitboard = 0x0000_0000_0000_00FF;
pub const RANK_2: Bitboard = 0x0000_0000_0000_FF00;
pub const RANK_3: Bitboard = 0x0000_0000_00FF_0000;
pub const RANK_4: Bitboard = 0x0000_0000_FF00_0000;
pub const RANK_5: Bitboard = 0x0000_00FF_0000_0000;
pub const RANK_6: Bitboard = 0x0000_FF00_0000_0000;
pub const RANK_7: Bitboard = 0x00FF_0000_0000_0000;
pub const RANK_8: Bitboard = 0xFF00_0000_0000_0000;
pub const RANKS: [Bitboard; 8] = [RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8];

pub const FILE_A: Bitboard = 0x0101_0101_0101_0101;
pub const FILE_B: Bitboard = 0x0202_0202_0202_0202;
pub const FILE_C: Bitboard = 0x0404_0404_0404_0404;
pub const FILE_D: Bitboard = 0x0808_0808_0808_0808;
pub const FILE_E: Bitboard = 0x1010_1010_1010_1010;
pub const FILE_F: Bitboard = 0x2020_2020_2020_2020;
pub const FILE_G: Bitboard = 0x4040_4040_4040_4040;
pub const FILE_H: Bitboard = 0x8080_8080_8080_8080;
pub const FILES: [Bitboard; 8] = [FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H];

// Hyperbola Quintessence Utility Lookups
const fn generate_primary_straight_rays(square: usize) -> Bitboard {
    // Staright rays are along the line of constant file
    let file = square % 8;
    FILES[file]
}

const fn generate_secondary_straight_rays(square: usize) -> Bitboard {
    // Staright rays are along the line of constant file
    let rank = square / 8;
    RANKS[rank]
}

const fn generate_primary_diagonal_rays(square: usize) -> Bitboard {
    // Primary diagonal: bottom-left -> top-right (a1-h8 direction),
    // i.e. squares where (rank - file) is constant.
    let file = (square % 8) as i32;
    let rank = (square / 8) as i32;
    let mut bitboard: Bitboard = 0;

    // Walk down-left to find the start of the diagonal
    let mut r = rank;
    let mut f = file;
    while r > 0 && f > 0 {
        r -= 1;
        f -= 1;
    }

    // Walk up-right from there, setting every square on the diagonal
    while r < 8 && f < 8 {
        let sq = (r * 8 + f) as usize;
        bitboard |= 1u64 << sq;
        r += 1;
        f += 1;
    }

    bitboard
}

const fn generate_secondary_diagonal_rays(square: usize) -> Bitboard {
    // Secondary diagonal (anti-diagonal): top-left -> bottom-right (a8-h1 direction),
    // i.e. squares where (rank + file) is constant.
    let file = (square % 8) as i32;
    let rank = (square / 8) as i32;
    let mut bitboard: Bitboard = 0;

    // Walk up-left to find the start of the anti-diagonal
    let mut r = rank;
    let mut f = file;
    while r < 7 && f > 0 {
        r += 1;
        f -= 1;
    }

    // Walk down-right from there, setting every square on the anti-diagonal
    while r >= 0 && f < 8 {
        let sq = (r * 8 + f) as usize;
        bitboard |= 1u64 << sq;
        r -= 1;
        f += 1;
    }

    bitboard
}

pub const PRIMARY_STRAIGHT_RAYS: [Bitboard; 64] = {
    let mut table: [Bitboard; 64] = [0; 64];
    let mut square = 0;
    while square < 64 {
        table[square] = generate_primary_straight_rays(square);
        square += 1;
    }
    table
};

pub const SECONDARY_STRAIGHT_RAYS: [Bitboard; 64] = {
    let mut table: [Bitboard; 64] = [0; 64];
    let mut square = 0;
    while square < 64 {
        table[square] = generate_secondary_straight_rays(square);
        square += 1;
    }
    table
};

pub const PRIMARY_DIAGONAL_RAYS: [Bitboard; 64] = {
    let mut table: [Bitboard; 64] = [0; 64];
    let mut square = 0;
    while square < 64 {
        table[square] = generate_primary_diagonal_rays(square);
        square += 1;
    }
    table
};

pub const SECONDARY_DIAGONAL_RAYS: [Bitboard; 64] = {
    let mut table: [Bitboard; 64] = [0; 64];
    let mut square = 0;
    while square < 64 {
        table[square] = generate_secondary_diagonal_rays(square);
        square += 1;
    }
    table
};

// For HQ lookup
pub const REVERSE_SQUARE_BB: [Bitboard; 64] = {
    let mut table = [0u64; 64];
    let mut sq = 0;
    while sq < 64 {
        table[sq] = (1u64 << sq).reverse_bits();
        sq += 1;
    }
    table
};

// Board starting position
pub const STARTING_BOARD: [Bitboard; 12] = [
    RANK_2, // White pawns
    RANK_1 & (FILE_B | FILE_G), // White knights
    RANK_1 & (FILE_C | FILE_F), // White bishops
    RANK_1 & (FILE_A | FILE_H), // White rooks
    RANK_1 & FILE_D, // White queen
    RANK_1 & FILE_E, // White king
    RANK_7, // White pawns
    RANK_8 & (FILE_B | FILE_G), // Black knights
    RANK_8 & (FILE_C | FILE_F), // Black bishops
    RANK_8 & (FILE_A | FILE_H), // Black rooks
    RANK_8 & FILE_D, // Black queen
    RANK_8 & FILE_E, // Black king
];

// Masks for checking castling legality faster
pub const WHITE_QS_ROOK: Bitboard = RANK_1 & FILE_A;
pub const WHITE_KS_ROOK: Bitboard = RANK_1 & FILE_H;
pub const BLACK_QS_ROOK: Bitboard = RANK_8 & FILE_A;
pub const BLACK_KS_ROOK: Bitboard = RANK_8 & FILE_H;

// Castling rights bitmasks
pub const WHITE_QS: u8 = 0b10000000;
pub const WHITE_KS: u8 = 0b01000000;
pub const BLACK_QS: u8 = 0b00100000;
pub const BLACK_KS: u8 = 0b00010000;

pub const CASTLING_RIGHTS_MASK: [u8; 64] = {
    // Default: 0b11110000 preserves all castling rights
    let mut table = [0b11110000; 64];

    // White King or Rook moves/captures
    table[Square::e1.0 as usize] &= !(WHITE_KS | WHITE_QS); // E1 clears all White castling
    table[Square::a1.0 as usize] &= !WHITE_QS;              // A1 clears White Queenside
    table[Square::h1.0 as usize] &= !WHITE_KS;              // H1 clears White Kingside

    // Black King or Rook moves/captures
    table[Square::e8.0 as usize] &= !(BLACK_KS | BLACK_QS); // E8 clears all Black castling
    table[Square::a8.0 as usize] &= !BLACK_QS;              // A8 clears Black Queenside
    table[Square::h8.0 as usize] &= !BLACK_KS;              // H8 clears Black Kingside

    table
};

pub const KINGSIDE_CASTLE_KING_DEST: [Bitboard; 2] = [RANK_1 & FILE_G, RANK_8 & FILE_G];
pub const QUEENSIDE_CASTLE_KING_DEST: [Bitboard; 2] = [RANK_1 & FILE_C, RANK_8 & FILE_C];

//pub const KINGSIDE_CASTLE_ROOK_XOR_DEST: [Bitboard; 2] = [RANK_1 & (FILE_H | FILE_F), RANK_8 & (FILE_H | FILE_F)];
//pub const QUEENSIDE_CASTLE_ROOK_XOR_DEST: [Bitboard; 2] = [RANK_1 & (FILE_A | FILE_D), RANK_8 & (FILE_A | FILE_D)];
pub const CASTLING_ROOK_XOR_DEST: [Bitboard; 4] = [
    RANK_1 & (FILE_A | FILE_D), // White Queenside (i.e. Castling::WhiteQueenside = 0)
    RANK_1 & (FILE_H | FILE_F), // White Kingside  (1)
    RANK_8 & (FILE_A | FILE_D), // Black Queenside (2)
    RANK_8 & (FILE_H | FILE_F), // Black Kingside (3)
];

pub const WHITE_KINGSIDE_FREE_SQUARES: Bitboard = RANK_1 & (FILE_F | FILE_G);
pub const WHITE_QUEENSIDE_FREE_SQUARES: Bitboard = RANK_1 & (FILE_B | FILE_C | FILE_D);
pub const BLACK_KINGSIDE_FREE_SQUARES: Bitboard = RANK_8 & (FILE_F | FILE_G);
pub const BLACK_QUEENSIDE_FREE_SQUARES: Bitboard = RANK_8 & (FILE_B | FILE_C | FILE_D);

pub const WHITE_KINGSIDE_SLIDER_ATTACKS: Bitboard = RANK_1 & (FILE_E | FILE_F | FILE_G);
pub const WHITE_QUEENSIDE_SLIDER_ATTACKS: Bitboard = RANK_1 & (FILE_E | FILE_D | FILE_C);
pub const BLACK_KINGSIDE_SLIDER_ATTACKS: Bitboard = RANK_8 & (FILE_E | FILE_F | FILE_G);
pub const BLACK_QUEENSIDE_SLIDER_ATTACKS: Bitboard = RANK_8 & (FILE_E | FILE_D | FILE_C);

pub const WHITE_KINGSIDE_ATTACKS_KING_OR_PAWN: Bitboard = RANK_2 & (FILE_D | FILE_E | FILE_F | FILE_G | FILE_H);
pub const BLACK_KINGSIDE_ATTACKS_KING_OR_PAWN: Bitboard = RANK_7 & (FILE_D | FILE_E | FILE_F | FILE_G | FILE_H);
pub const WHITE_KINGSIDE_ATTACKS_KNIGHT: Bitboard = (RANK_2 & (FILE_C | FILE_D | FILE_E | FILE_G | FILE_H)) 
                                                    | (RANK_3 & (FILE_D | FILE_E | FILE_F | FILE_G | FILE_H));
pub const BLACK_KINGSIDE_ATTACKS_KNIGHT: Bitboard = (RANK_7 & (FILE_C | FILE_D | FILE_E | FILE_G | FILE_H)) 
                                                    | (RANK_6 & (FILE_D | FILE_E | FILE_F | FILE_G | FILE_H));

pub const WHITE_QUEENSIDE_ATTACKS_KING_OR_PAWN: Bitboard = RANK_2 & (FILE_B | FILE_C | FILE_D | FILE_E | FILE_F);
pub const BLACK_QUEENSIDE_ATTACKS_KING_OR_PAWN: Bitboard = RANK_7 & (FILE_B | FILE_C | FILE_D | FILE_E | FILE_F);
pub const WHITE_QUEENSIDE_ATTACKS_KNIGHT: Bitboard = (RANK_2 & (FILE_A | FILE_B | FILE_C | FILE_E | FILE_F | FILE_G)) 
                                                    | (RANK_3 & (FILE_B | FILE_C | FILE_D | FILE_E | FILE_F));
pub const BLACK_QUEENSIDE_ATTACKS_KNIGHT: Bitboard = (RANK_7 & (FILE_A | FILE_B | FILE_C | FILE_E | FILE_F | FILE_G)) 
                                                    | (RANK_6 & (FILE_B | FILE_C | FILE_D | FILE_E | FILE_F));

// For constructing moves (32-bit move words)
pub const PROMOTE_QUEEN_AND_CAPTURE:  u32 = 1 << 17 | 1 << 22;
pub const PROMOTE_ROOK_AND_CAPTURE:   u32 = 1 << 17 | 1 << 23;
pub const PROMOTE_KNIGHT_AND_CAPTURE: u32 = 1 << 17 | 1 << 24;
pub const PROMOTE_BISHOP_AND_CAPTURE: u32 = 1 << 17 | 1 << 25;