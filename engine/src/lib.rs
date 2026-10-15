// Expose the generator module
pub mod generator;
pub mod engine;

use crate::generator::{movegen::Generator};

pub fn perft(generator: &mut Generator, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }

    let moves = generator.get_moves(false);

    if depth == 1 {
        //for mv in &moves {
        //    mv.print();
        //    print!(": 1\n");
        //}
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

pub fn perft_divide(generator: &mut Generator, depth: u32) {
    if depth == 0 {
        return;
    }

    let moves = generator.get_moves(false);
    let mut total = 0u64;

    for mv in moves {
            let undo = generator.b.make_move(mv);
            let count = perft(generator, depth - 1);
            total += count;
            generator.b.unmake_move(undo);
            mv.print();
            println!(": {count}");
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
    const SJESA: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10";
    const POS3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    const POS5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    const POS4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";

    #[test]
    fn pos4_depth_1_to_4() {
        assert_eq!(run(POS4, 1), 6);
        assert_eq!(run(POS4, 2), 264);
        assert_eq!(run(POS4, 3), 9_467);
        assert_eq!(run(POS4, 4), 422_333);
    }

    #[test]
    fn pos5_depth_1_to_4() {
        assert_eq!(run(POS5, 1), 44);
        assert_eq!(run(POS5, 2), 1_486);
        assert_eq!(run(POS5, 3), 62_379);
        assert_eq!(run(POS5, 4), 2_103_487);
    }

    #[test]
    fn pos3_depth_1_to_6() {
        assert_eq!(run(POS3, 1), 14);
        assert_eq!(run(POS3, 2), 191);
        assert_eq!(run(POS3, 3), 2_812);
        assert_eq!(run(POS3, 4), 43_238);
        assert_eq!(run(POS3, 5), 674_624);
        assert_eq!(run(POS3, 6), 11_030_083);
    }

    #[test]
    fn sjesa_depth_1_to_4() {
        assert_eq!(run(SJESA, 1), 46);
        assert_eq!(run(SJESA, 2), 2_079);
        assert_eq!(run(SJESA, 3), 89_890);
        assert_eq!(run(SJESA, 4), 3_894_594);
    }

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