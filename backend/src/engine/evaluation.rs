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
        let mut piece_value: [i32; 2] = [0; 2];
        let mut pst_value: [i32; 2] = [0; 2];

        while bb != 0 {
            let sq = bb.trailing_zeros() as usize;
            piece_value[0] += MG_VALUES[piece];
            piece_value[1] += EG_VALUES[piece];

            pst_value[0] += MG_PST[piece][sq ^ 56];
            pst_value[1] += EG_PST[piece][sq ^ 56];
            
            bb &= bb - 1; // Clear the least significant bit
        }
        score += (piece_value[0] as f32 * mphase + piece_value[1] as f32 * phase) + (pst_value[0] as f32 * mphase + pst_value[1] as f32 * phase);

        let mut piece_value: [i32; 2] = [0; 2];
        let mut pst_value: [i32; 2] = [0; 2];

        // Black (mirror the square vertically)
        let mut bb = board.bitboards[6 + piece];
        while bb != 0 {
            let sq = bb.trailing_zeros() as usize;
            piece_value[0] += MG_VALUES[piece];
            piece_value[1] += EG_VALUES[piece];
            pst_value[0] += MG_PST[piece][sq];
            pst_value[1] += EG_PST[piece][sq];
            bb &= bb - 1; // Clear the least significant bit
        }
        score -= ((piece_value[0] as f32 * mphase + piece_value[1] as f32 * phase) + (pst_value[0] as f32 * mphase + pst_value[1] as f32 * phase)).abs();
    }

    score
}