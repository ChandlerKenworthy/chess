use crate::engine::evaluation::{INFINITY, evalaute};
use crate::generator::movegen::{Generator};
use crate::generator::{Color, Move};

pub fn negamax(generator: &mut Generator, depth: i32, mut alpha: i32, beta: i32) -> (i32, Option<Move>) {
    if depth == 0 {
        let color = match generator.b.to_move {
            Color::White => 1,
            Color::Black => -1,
        };
        return (color * evalaute(&generator.b), None);
    }

    // Generate all moves for this position
    let moves = generator.get_moves();

    // Terminal state handling (0 legal moves available)
    if moves.is_empty() {
        if generator.is_in_check(generator.b.to_move) {
            // Checkmate: Adjust mate score by depth so nearer mates score higher
            // Subtracting depth ensures quicker checkmates are preferred
            return (-INFINITY - depth, None);
        } else {
            // Stalemate
            return (0, None);
        }
    }

    let mut best_move: Option<Move> = None;
    let mut best_score = -INFINITY;

    for mv in moves {
        let undo = generator.b.make_move(mv);
        // For an explanation of Alpha-beta algorithm see:
        // https://chessprogramming.org/Alpha-Beta
        let (score, _) = negamax(generator, depth - 1, -beta, -alpha);
        let score = -score;

        generator.b.unmake_move(undo);
        if score > best_score {
            best_score = score;
            best_move = Some(mv);
        }

        // Update alpha (the best guaranteed score for the current player)
        if best_score > alpha {
            alpha = best_score;
        }

        // Alpha-Beta Cutoff: opponent won't allow this branch
        if alpha >= beta {
            break;
        }
    }
    (best_score, best_move)
}
