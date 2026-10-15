pub mod board;
pub mod movegen;
pub mod piece;
pub mod bitboard;
pub mod game_move;
pub mod magic; // magic bitboard implementation

// Re-export commonly moved types for convenience
pub use board::{Board, Square};
pub use game_move::Move;
pub use piece::{Piece, Color, PieceType, Castling};
pub use bitboard::Bitboard;