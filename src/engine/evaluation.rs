use crate::generator::{Board};

pub const INFINITY: i32 = 1_000_000_000;

// Note: all evaluations are in the units of centipawns (so a score of +100 = +1 pawn)
pub fn evalaute(board: &Board) -> i32 {
    let white = 0;
    let black = 6;

    let values: [i32; 6] = [
        100, // pawn
        320, // knight
        330, // bishop
        500, // rook
        900, // queen
        0,   // king
    ];

    let mut score = 0;

    for piece in 0..6 {
        score += board.bitboards[white + piece].count_ones() as i32 * values[piece];
        score -= board.bitboards[black + piece].count_ones() as i32 * values[piece];
    }

    score
}