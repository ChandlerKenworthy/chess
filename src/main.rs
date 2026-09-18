use chess::generator::bitboard::{FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, PRIMARY_DIAGONAL_RAYS, RANK_1, RANK_2, east, north, south};
use chess::generator::movegen::Generator;
use chess::generator::{Board, Square, bitboard::print_bitboard};
use chess::generator::{Bitboard, Move, Piece};

fn main() {

    let mut b = Board::new();
    b.clear();
    b.bitboards[Piece::WhiteKing as usize] = 1 << Square::e1;
    b.bitboards[Piece::WhiteRook as usize] = RANK_1 & (FILE_A | FILE_H);
    b.bitboards[Piece::BlackKnight as usize] = RANK_2 & FILE_H;
    b.print();

    let mut g: Generator = Generator::new(b);
    let moves: Vec<Move> = g.get_moves(); // generates and returns the vector of moves
    
    for mv in moves {
        mv.print();
    }


    /* let s = Square::new(35);
    println!("{s}");

    print_bitboard(PRIMARY_DIAGONAL_RAYS[35]); */
    

}