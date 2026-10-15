use chess::generator::bitboard::KIWIPETE;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use chess::generator::Board;
use chess::engine::Engine;


fn bench_kiwipete_depth_5(c: &mut Criterion) {
    let board = Board::from_fen(KIWIPETE).expect("Invalid FEN");
    let mut engine = Engine::new(board);

    c.bench_function("kiwipete depth 5", |b| {
        b.iter(|| {            
            // black_box prevents the compiler from aggressively optimizing away the result
            black_box(engine.get_best_move(5))
        });
    });
}

criterion_group!(benches, bench_kiwipete_depth_5);
criterion_main!(benches);