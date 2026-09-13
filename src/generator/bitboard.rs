pub type Bitboard = u64; // Bitboards are unsigned 64-bit integers

// Rank and file definitions
pub const RANK_1: Bitboard = 0x0000_0000_0000_00FF;
pub const RANK_2: Bitboard = 0x0000_0000_0000_FF00;
pub const RANK_3: Bitboard = 0x0000_0000_00FF_0000;
pub const RANK_4: Bitboard = 0x0000_0000_FF00_0000;
pub const RANK_5: Bitboard = 0x0000_00FF_0000_0000;
pub const RANK_6: Bitboard = 0x0000_FF00_0000_0000;
pub const RANK_7: Bitboard = 0x00FF_0000_0000_0000;
pub const RANK_8: Bitboard = 0xFF00_0000_0000_0000;

pub const FILE_A: Bitboard = 0x0101_0101_0101_0101;
pub const FILE_B: Bitboard = 0x0202_0202_0202_0202;

// todo: these are all wrong
pub const FILE_C: Bitboard = 0x0303_0303_0303_0303;
pub const FILE_D: Bitboard = 0x0404_0404_0404_0404;
pub const FILE_E: Bitboard = 0x0505_0505_0505_0505;
pub const FILE_F: Bitboard = 0x0606_0606_0606_0606;
pub const FILE_G: Bitboard = 0x0707_0707_0707_0707;
pub const FILE_H: Bitboard = 0x0808_0808_0808_0808;