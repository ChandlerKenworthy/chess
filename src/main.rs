use chess::generator::magic::rook_attacks;
use chess::generator::{Board, Square, magic};
use chess::engine::Engine;
use chess::generator::bitboard::{FILE_A, KIWIPETE, RANK_1, RANK_5, print_bitboard};

fn main() {
    magic::init();

    let s = Square::a1;
    let occ = FILE_A & (RANK_1 | RANK_5);

    print_bitboard(RANK_1 & FILE_A);
    print_bitboard(occ);

    let x = rook_attacks(s.index() as usize, occ);
    print_bitboard(x);
    

}

