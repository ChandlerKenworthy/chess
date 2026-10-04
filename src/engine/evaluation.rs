use crate::{engine::pst::{EG_PST, EG_VALUES, MG_PST, MG_VALUES}, generator::Board};

pub const INFINITY: i32 = 1_000_000_000;

// Note: all evaluations are in the units of centipawns (so a score of +100 = +1 pawn)
pub fn evaluate(board: &Board) -> f32 {
    let mut score = 0.0;

    let phase = board.get_phase();
    let mphase = 1.0 - phase;

    for piece in 0..6 {
        // White
        let mut bb = board.bitboards[piece];
        while bb != 0 {
            let sq = bb.trailing_zeros() as usize;
            let piece_value = MG_VALUES[piece] as f32 * mphase + EG_VALUES[piece] as f32 * phase;
            let pst_value = MG_PST[piece][sq ^ 56] as f32 * mphase + EG_PST[piece][sq ^ 56] as f32 * phase;
            score += piece_value + pst_value;
            bb &= bb - 1; // Clear the least significant bit
        }

        // Black (mirror the square vertically)
        let mut bb = board.bitboards[6 + piece];
        while bb != 0 {
            let sq = bb.trailing_zeros() as usize;
            let piece_value = MG_VALUES[piece] as f32 * mphase + EG_VALUES[piece] as f32 * phase;
            let pst_value = MG_PST[piece][sq] as f32 * mphase + EG_PST[piece][sq] as f32 * phase;
            score -= (piece_value + pst_value).abs();
            bb &= bb - 1; // Clear the least significant bit
        }
    }

    score
}