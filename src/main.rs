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
    
    moves[0].print();
    b.make_move(moves[0]);

    println!();
    b.print();
    

}

/* fn perft(board: &mut Board, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut g = Generator::new(board.clone()); // or pass &*board if Generator can borrow
    let moves = g.get_moves();

    if depth == 1 {
        return moves.len() as u64;
    }

    let mut nodes = 0u64;
    for mv in moves {
        let undo = board.make_move(mv);   // make_move returns undo info
        nodes += perft(board, depth - 1);
        board.unmake_move(mv, undo);
    }
    nodes
}

fn perft_divide(board: &Board, depth: u32) {
    let mut g = Generator::new(board.clone());
    let moves = g.get_moves();

    let mut total = 0u64;
    for mv in moves {
        let mut next = board.clone();
        next.make_move(mv);
        let count = perft(&next, depth - 1);
        total += count;
        mv.print();
        print!(": {count}\n");
    }
    println!("Total: {}", total);
} */