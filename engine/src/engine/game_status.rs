use crate::{engine::{evaluation::INFINITY}, generator::Color};
use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum DrawReason {
    FiftyMoveRule,
    ThreefoldRepetition,
    InsufficientMaterial,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GameStatus {
    Ongoing,
    Checkmate { winner: Color },
    Stalemate,
    Draw(DrawReason)
}

impl GameStatus {
    pub fn score(&self) -> f32 {
        match self {
            GameStatus::Checkmate { .. } => -INFINITY as f32,
            _ => 0.0,
        }
    }
}
