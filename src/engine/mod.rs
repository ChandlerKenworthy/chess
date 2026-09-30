pub mod evaluation;
pub mod search;

use crate::engine::search::search;
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

    pub fn get_best_move(&mut self) -> Move {
        search(&mut self.generator).unwrap()
    }
}
