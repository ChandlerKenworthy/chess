use crate::generator::Color;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GameStatus {
    Ongoing,
    Checkmate { winner: Color },
    Stalemate
}
