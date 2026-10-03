use crate::engine::evaluation::evalaute;
use crate::generator::movegen::{Generator};
use crate::generator::{Color, Move};

pub fn negamax(generator: &mut Generator, depth: i32) -> (i32, Option<Move>) {
    if depth == 0 {
        let color = match generator.b.to_move {
            Color::White => 1,
            Color::Black => -1,
        };
        return (color * evalaute(&generator.b), None);
    }

    // Generate all moves for this position
    let moves = generator.get_moves();

    let mut best_move: Option<Move> = None;
    let mut best_score = i32::MIN;

    for mv in moves {
        let undo = generator.b.make_move(mv);
        let (score, _) = negamax(generator, depth - 1);
        let score = -score;

        generator.b.unmake_move(undo);
        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }
    }

    (best_score, best_move)
}

pub fn search(generator: &mut Generator) -> Option<Move> {
    let moves = generator.get_moves();

    let mut best_move = None;
    let mut best_score = i32::MIN;
    let color: i32 = match generator.b.to_move {
        Color::White => 1,
        Color::Black => -1
    };

    for mv in moves {
        let undo = generator.b.make_move(mv);

        // Score is always positive e.g. evaluate produces large -ve for good black position
        // gets flipped here
        let score = evalaute(&generator.b) * color;

        generator.b.unmake_move(undo);

        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }
    }

    println!("Best evaluation was: {}", best_score);

    best_move
}