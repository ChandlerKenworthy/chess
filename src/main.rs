use chess::generator::bitboard::{FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, PRIMARY_DIAGONAL_RAYS, RANK_1, RANK_2, RANK_3, RANK_4, RANK_6, RANK_7, RANK_8, east, north, south};
use chess::generator::movegen::Generator;
use chess::generator::{Board, Square, bitboard::print_bitboard};
use chess::generator::{Bitboard, Move, Piece, PieceType};

fn main() {

    let mut b = Board::new();
    /* b.clear();
    b.bitboards[Piece::BlackKing as usize] = RANK_8 & FILE_C;
    b.bitboards[Piece::WhiteKing as usize] = 1 << Square::e1;
    b.bitboards[Piece::WhiteRook as usize] = RANK_1 & (FILE_A | FILE_H);
    b.bitboards[Piece::BlackKnight as usize] = RANK_2 & FILE_H;
    b.bitboards[Piece::WhitePawn as usize] = RANK_2 & FILE_D;
    b.bitboards[Piece::BlackPawn as usize] = RANK_4 & FILE_C;
    b.print();

    // Make whites moves i.e. double pawn push
    let dblmv = Move::new_with_flags(RANK_2 & FILE_D, RANK_4 & FILE_D, PieceType::Pawn, 0x10000);
    println!("Move is double-push? {}", dblmv.is_double_pawn_push());
    b.make_move(dblmv);
 */
    //b.print();

    // Make the moves to re-produce generator errors here
    //let mv = Move::new(
    //    RANK_2 & FILE_A, 
    //    RANK_3 & FILE_A,
    //    PieceType::Pawn,
    //    false);
    //b.make_move(mv);

    //b.make_move(Move::new(
    //    RANK_8 & FILE_B,
    //    RANK_6 & FILE_C, 
    //    PieceType::Knight, false));
    //b.print();

    // Copies an instance of the board in here
    let mut g: Generator = Generator::new(b);

    let depth = 4;
    perft_divide(&mut g, depth);


    //let perft_res = perft(&mut g, &mut b, 3);
    //println!("Perft 3 = {perft_res}");
    

}

fn perft(generator: &mut Generator, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = generator.get_moves();
    //generator.b.print();

    if depth == 1 {
/*         for &mv in &moves {
            print!("  ");
            mv.print();
            println!();
        } */
        return moves.len() as u64;
    }

    let mut nodes = 0u64;
    for mv in moves {
        let undo = generator.b.make_move(mv);
        nodes += perft(generator, depth - 1);
        generator.b.unmake_move(undo);
    }
    nodes
}

fn perft_divide(generator: &mut Generator, depth: u32) {
    if depth == 0 {
        return;
    }

    let moves = generator.get_moves();
    let mut total = 0u64;

    for mv in moves {
        let undo = generator.b.make_move(mv);
        let count = perft(generator, depth - 1);
        total += count;
        generator.b.unmake_move(undo);
        print!("Root move: ");
        mv.print();
        print!(": {count}\n");
    }
    println!("Total: {}", total);
}