use serde::Deserialize;

#[derive(Deserialize)]
pub struct MoveRequest {
    pub from: String, // a1, c4 etc.
    pub to: String
}