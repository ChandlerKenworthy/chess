use crate::generator::{Board};

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