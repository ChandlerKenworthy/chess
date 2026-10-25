use chess_engine::{engine::game_status::GameStatus, generator::Color};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MoveRequest {
    pub from: String, // a1, c4 etc.
    pub to: String
}

#[derive(Serialize)]
pub struct GameStateResponse {
    pub fen: String,
    pub winner: Option<Color>,
    pub status: GameStatus
}

#[derive(Serialize)]
pub struct MakeMoveResponse {
    pub fen: String, // Resulting FEN after the move was (or wasn't due to terminal game state) made
    pub status: GameStatus,
    pub move_was_legal: bool, // Did the user/computer attempt to make an illegal move
}