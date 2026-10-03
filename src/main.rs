use chess::generator::movegen::Generator;
use chess::generator::{Board};
use chess::{perft};
use chess::engine::{Engine};
use std::{env, println};

fn main() {

    const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    //const POS: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    
    let mut b = Board::from_fen(KIWIPETE).expect("Invalid FEN");
    b.print();

    let mut eng = Engine::new(b);
    let mv = eng.get_best_move(4);

    mv.print();
    println!();

}

