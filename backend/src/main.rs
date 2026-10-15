use chess::generator::Board;
use chess::engine::Engine;
use chess::generator::bitboard::KIWIPETE;

fn main() {

    let board = Board::from_fen(KIWIPETE).expect("Invalid FEN");
    let mut engine = Engine::new(board);
    let mv = engine.get_best_move(5);

    mv.print();
    println!();
    

}

