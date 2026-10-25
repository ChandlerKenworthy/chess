use std::sync::Arc;
use tokio::sync::Mutex;
use chess_engine::{engine::Engine, generator::{Board, board::UndoState}};

pub struct AppState {
    pub undo: Option<UndoState>,
    pub engine: Engine
}

impl AppState {
    pub fn new() -> Self {
        let b = Board::new();

        Self {
            undo: None,
            engine: Engine::new(b)
        }
    }

    pub fn reset(&mut self) {
        let b = Board::new();
        self.undo = None;
        self.engine = Engine::new(b)
    }
}

pub type SharedState = Arc<Mutex<AppState>>;