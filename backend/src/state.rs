use std::sync::Arc;
use tokio::sync::Mutex;
use chess_engine::{generator::{Board, movegen::Generator}};

pub struct AppState {
    pub board: Board,
    pub generator: Generator,
    //pub engine: Engine,
}

impl AppState {
    pub fn new() -> Self {
        let b = Board::new();

        Self {
            board: b,
            generator: Generator::new(b),
            //engine: 0,
        }
    }
}

pub type SharedState = Arc<Mutex<AppState>>;