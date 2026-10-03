pub mod evaluation;
pub mod search;

use crate::engine::search::{negamax};
use crate::engine::evaluation::{INFINITY};
use crate::generator::{Board, Move};
use crate::generator::movegen::{Generator};

pub struct Engine {
    pub generator: Generator
}

impl Engine {
    pub fn new(board: Board) -> Self {
        Self {
            generator: Generator::new(board),
        }
    }

    pub fn get_best_move(&mut self, depth: i32) -> Move {
        let (score, mv) = negamax(&mut self.generator, depth, -INFINITY, INFINITY);

        // Format the score nicely
        let score = if score >= 0 {
            format!("+{:.1}", score as f32 / 100.0)
        } else {
            format!("-{:.1}", -score as f32 / 100.0)
        };

        println!("Evaluation: {}", score);
        mv.expect("No available moves")
        
    }
}
