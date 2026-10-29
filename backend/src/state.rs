use std::sync::Arc;
use tokio::sync::Mutex;
use chess_engine::{engine::{Engine, game_status::GameStatus}, generator::{Board, Color, board::UndoState}};

use crate::requests::GameStateResponse;

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

    pub fn from_fen(&mut self, fen: String) -> Result<GameStateResponse, String> {
        let b = Board::from_fen(&fen);

        match b {
            Some(board) => {
                self.undo = None;
                self.engine = Engine::new(board);

                // Generate the legal moves
                let moves = self.engine.generator.get_moves();

                // Determine the game state
                let game_state = self.engine.get_game_status(&moves);

                // Fling it back to the frontend
                let winner = match game_state {
                    GameStatus::Stalemate => Some(!self.engine.generator.b.to_move),
                    GameStatus::Checkmate{ winner } => Some(winner),
                    GameStatus::Ongoing => None
                };

                let response = GameStateResponse {
                    fen: fen,
                    winner: winner,
                    status: game_state
                };

                Ok(response)
            },
            None => Err("Failed to Parse FEN".to_string())
        }
    }
}

pub type SharedState = Arc<Mutex<AppState>>;