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
        println!("");
    }
    println!("   A  B  C  D  E  F  G  H  \n");
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