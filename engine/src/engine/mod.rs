pub mod evaluation;
pub mod search;
pub mod pst;
pub mod game_status;

use crate::generator::{Board};
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
}
