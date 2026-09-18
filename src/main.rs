use chess::generator::bitboard::{FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, PRIMARY_DIAGONAL_RAYS, RANK_2, east, north, south};
use chess::generator::movegen::Generator;
use chess::generator::{Board, Square, bitboard::print_bitboard};
use chess::generator::{Bitboard, Move, Piece};

fn main() {

    let mut b = Board::new();
    b.bitboards[Piece::WhitePawn as usize] = b.bitboards[Piece::WhitePawn as usize] ^ (RANK_2 & (FILE_E | FILE_D));
    b.print();

    //let s = Square::new(43);
    //println!("{s}");

    let mut g: Generator = Generator::new(b);
    let moves: Vec<Move> = g.get_moves(); // generates and returns the vector of moves
    
    for mv in moves {
        mv.print();
    }


    /* let s = Square::new(35);
    println!("{s}");

    print_bitboard(PRIMARY_DIAGONAL_RAYS[35]); */
    

}