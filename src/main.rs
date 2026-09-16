use chess::generator::movegen::Generator;
use chess::generator::{Board, Square, bitboard::print_bitboard};
use chess::generator::{self, Move};

fn main() {

    let b = Board::new();
    //b.print();

    //let s = Square::new(43);
    //println!("{s}");

    let mut g: Generator = Generator::new(b);
    let moves: Vec<Move> = g.get_moves();
    
    for mv in moves {
        print_bitboard(mv.to());
    }
    

}