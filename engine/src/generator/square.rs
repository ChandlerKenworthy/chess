use crate::generator::Bitboard;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Square(pub u8);

impl Square {
    pub const fn new(index: u32) -> Self {
        debug_assert!(index < 64);
        Self(index as u8)
    }

    pub const fn index(self) -> u8 {
        self.0
    }

    pub fn bb(&self) -> Bitboard {
        1u64 << self.0
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file = (b'a' + self.0 % 8) as char;
        let rank = (b'1' + self.0 / 8) as char;

        write!(f, "{file}{rank}")
    }
}

impl Square {
    pub fn from_str(square: &str) -> Option<u32> {
        let bytes = square.as_bytes();

        if bytes.len() != 2 {
            return None;
        }

        let file = bytes[0];
        let rank = bytes[1];

        if !(b'a'..=b'h').contains(&file)
            || !(b'1'..=b'8').contains(&rank)
        {
            return None;
        }

        let file_index = file - b'a';
        let rank_index = rank - b'1';

        Some((rank_index * 8 + file_index) as u32)
    }
}