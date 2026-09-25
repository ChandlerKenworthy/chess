use chess::generator::bitboard::{FILE_A, FILE_B, FILE_C, FILE_D, FILE_E, FILE_F, FILE_G, FILE_H, PRIMARY_DIAGONAL_RAYS, RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7, RANK_8, east, north, south};
use chess::generator::movegen::Generator;
use chess::generator::{Board, Square, bitboard::print_bitboard};
use chess::generator::{Bitboard, Move, Piece, PieceType};
use chess::{perft, perft_divide};

use std::time::Instant;
use std::env;

fn main() {
    let depth: u32 = env::args()
        .nth(1)
        .expect("Please provide a depth argument, e.g., 'cargo run -- 6'")
        .parse()
        .expect("Depth must be a valid positive number");

    const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    let b = Board::from_fen(KIWIPETE).expect("Invalid FEN");
    let mut g: Generator = Generator::new(b);

    let start = Instant::now();

    let nodes = perft(&mut g, depth);

    let elapsed = start.elapsed();

    println!("Nodes: {nodes}");
    println!("Time: {:.3?}", elapsed);
    println!("NPS: {:.2} Mn/s", nodes as f64 / elapsed.as_secs_f64() / 1_000_000.0);


}

