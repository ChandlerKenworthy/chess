use serde::Deserialize;

#[derive(Deserialize)]
pub struct MoveRequest {
    pub from: String, // a1, c4 etc.
    pub to: String,
    pub piece: String, // e.g. "wR" for white rook, "bN" for black knight, etc.
}