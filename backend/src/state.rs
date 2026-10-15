use std::sync::Arc;
use tokio::sync::Mutex;
use chess_engine::generator::{Board, board::UndoState, movegen::Generator};

pub struct AppState {
    pub board: Board,
    pub generator: Generator,
    pub undo: Option<UndoState>,
    //pub engine: Engine,
}

impl AppState {
    pub fn new() -> Self {
        let b = Board::new();

        Self {
            board: b,
            generator: Generator::new(b),
            undo: None,
            //engine: 0,
        }
    }
}

pub type SharedState = Arc<Mutex<AppState>>;