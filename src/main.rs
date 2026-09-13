use std::println;

use chess::generator::{Bitboard, Board, Move, Square};
use chess::generator::bitboard::{
    RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8,
    FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H
};

fn print_bitboard(bb: Bitboard) {
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
    println!("   A  B  C  D  E  F  G  H. ");
}

fn main() {

    let mut b = Board::new();
    b.bitboards[3] = 1u64 << 16;

    //b.print();

    //print_bitboard(RANK_1);
    //print_bitboard(RANK_2);
    //print_bitboard(RANK_3);
    //print_bitboard(RANK_4);
    //print_bitboard(RANK_5);
    //print_bitboard(RANK_6);
    //print_bitboard(RANK_7);
    //print_bitboard(RANK_8);
    print_bitboard(FILE_A);
    print_bitboard(FILE_B);
    print_bitboard(FILE_C);
    print_bitboard(FILE_D);
    print_bitboard(FILE_E);
    print_bitboard(FILE_F);
    print_bitboard(FILE_G);
    print_bitboard(FILE_H);

    
    let from = Square::new(41);
    println!("{from}");


    let from: u64 = 1 << 1;
    let to: u64 = 1 << 3;
    let m = Move::new(from, to);

    println!("You originated at {}", Square::new(m.from_index()))
    //print_bitboard(from | to | (1u64 << 8));

}