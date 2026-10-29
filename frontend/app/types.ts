export interface MakeMoveResponse {
    fen: string;
    status:
        | { type: "ongoing" }
        | { type: "stalemate" }
        | { type: "checkmate"; winner: "white" | "black" };
    move_was_legal: boolean;
    eval: number;
}

export interface MoveRequest {
  from: string;
  to: string;
}

export interface FenRequest {
    fen: string;
}

export type GameStatus =
    | { type: "ongoing" }
    | { type: "stalemate" }
    | { type: "checkmate"; winner: "white" | "black" };

export interface GameState {
    fen: string,
    winner: "white" | "black" | undefined,
    status: GameStatus,
    evaluation: number,
}

export interface GameStateResponse {
    fen: string,
    winner: "white" | "black" | undefined,
    status: GameStatus,
}

export const startGameState = {
    fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
    winner: undefined,
    status: {type: "ongoing"},
    evaluation: 0.0,
} as GameState;