use crate::generator::Bitboard;

pub struct KingSafety {
    pub checkers: Bitboard,
    pub pinned: Bitboard,
    pub pin_rays: [Bitboard; 64],
}