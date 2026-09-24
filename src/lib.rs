// Expose the generator module
pub mod generator;
use crate::generator::movegen::Generator;

fn perft(generator: &mut Generator, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = generator.get_moves();

    if depth == 1 {
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
        //print!("Root move: ");
        mv.print();
        print!(": {count}\n");
    }
    println!("Total: {}", total);
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

use super::*;
    use crate::generator::Board;

    fn run(fen: &str, depth: u32) -> u64 {
        let b = Board::from_fen(fen).expect("Invalid FEN"); // assumes FEN is valid
        let mut g = Generator::new(b);
        perft(&mut g, depth)
    }

    const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";

    #[test]
    fn kiwipete_depth_1_to_4() {
        assert_eq!(run(KIWIPETE, 1), 48);
        assert_eq!(run(KIWIPETE, 2), 2_039);
        assert_eq!(run(KIWIPETE, 3), 97_862);
        assert_eq!(run(KIWIPETE, 4), 4_085_603);
    }

    #[test]
    fn startpos_depth_1_to_5() {
        //assert_eq!(run(START, 1), 20);
        //assert_eq!(run(START, 2), 400);
        //assert_eq!(run(START, 3), 8_902);
        //assert_eq!(run(START, 4), 197_281);
        assert_eq!(run(START, 5), 4_865_609);
    }

    /* #[test]
    #[ignore] // slow; run with `cargo test --release -- --ignored`
    fn startpos_depth_6() {
        assert_eq!(run(START, 6), 119_060_324);
    } */
}