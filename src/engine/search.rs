use crate::engine::evaluation::{INFINITY, evaluate};
use crate::generator::movegen::{Generator, MoveList};
use crate::generator::{Color, Move, PieceType};
use std::cmp::Reverse;

const PROMOTION_SCORE: i32 = 100_000;
const CAPTURE_SCORE: i32 = 10_000;
// Pawn, knight, bishop, rook, queen, king
const PIECE_VALUES: [i32; 6] = [100, 320, 330, 500, 900, 20_000];

// TODO: Should this live here or in evaluation or somewhere else?
fn score_move(mv: Move, g: &Generator) -> i32 {
    // TODO: This can be made simpler by simply having an exhaustive list
    // of the bits in certain positions of the move to get a score
    // as it is independent of the side

    let mut score = 0;

    if mv.is_capture() {
        // MVV-LVA: prefer valuable victims and cheap attakers
        // e.g. pawns capturing queens is very good

        // If no piece then move must have been an en-passant
        let victim = mv.captured_piece().unwrap_or(PieceType::Pawn);

        // TODO: Cleanup via an if-let
        score += CAPTURE_SCORE
            + 10 * PIECE_VALUES[victim as usize]
            - PIECE_VALUES[mv.piece() as usize];
    }

    if mv.is_promotion() {
        score += PROMOTION_SCORE;
    }

    score
}

fn order_moves(moves: &mut MoveList, g: &Generator) {
    // Order moves with the highest score first so alpha-beta prunes them quickly
    moves.sort_by_key(|&mv| Reverse(score_move(mv, g)));
}

pub fn negamax(generator: &mut Generator, depth: i32, mut alpha: f32, beta: f32) -> (f32, Option<Move>) {
    if depth == 0 {
        let color = match generator.b.to_move {
            Color::White => 1,
            Color::Black => -1,
        };
        return (color as f32 * evaluate(&generator.b), None);
    }

    // Generate all moves for this position
    let mut moves = generator.get_moves();
    
    // If depth > 1, order the moves to prune faster
    if depth > 1 {    
        order_moves(&mut moves, generator);
    }

    // Terminal state handling (0 legal moves available)
    if moves.is_empty() {
        if generator.is_in_check(generator.b.to_move) {
            // Checkmate: Adjust mate score by depth so nearer mates score higher
            // Subtracting depth ensures quicker checkmates are preferred
            return ((-INFINITY - depth) as f32, None);
        } else {
            // Stalemate
            return (0.0, None);
        }
    }

    let mut best_move: Option<Move> = None;
    let mut best_score = -INFINITY as f32;

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
