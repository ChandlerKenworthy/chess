use chess::generator::bitboard::{FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, PRIMARY_DIAGONAL_RAYS, RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8, east, north, south};
use chess::generator::movegen::Generator;
use chess::generator::{Board, Square, bitboard::print_bitboard};
use chess::generator::{Bitboard, Move, Piece, PieceType};
use chess::perft_divide;

fn main() {

    let startfen: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let kiwipete: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    let pos3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";

    // kiwipete --> a2a3
    let x: &str = "8/2p5/3p4/KP5r/4P2k/8/6p1/7R b - - 1 3";

    //let mut b = Board::new();
    let mut b = Board::from_fen(pos3).expect("Invalid FEN");
    b.print();

    let mut g: Generator = Generator::new(b);

    let depth = 6;
    perft_divide(&mut g, depth);


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

/*     b.make_move(Move::new_with_flags(
        RANK_2 & FILE_A,
        RANK_4 & FILE_A, 
        PieceType::Pawn, 0x10000));

    b.make_move(Move::new_with_flags(
        RANK_7 & FILE_B,
        RANK_5 & FILE_B, 
        PieceType::Pawn, 0x10000)); */

    //b.make_move(Move::new(
    //    RANK_4 & FILE_A,
    //    RANK_5 & FILE_B, 
    //    PieceType::Pawn, true));


    //b.print();

    //let perft_res = perft(&mut g, &mut b, 3);
    //println!("Perft 3 = {perft_res}");
    

}

