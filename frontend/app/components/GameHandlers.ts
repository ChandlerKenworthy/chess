import axios from "axios";
import { type GameStateResponse, type GameState, type MakeMoveResponse, type MoveRequest, type FenRequest } from "~/types";

function updateGameState(
    setGame: React.Dispatch<React.SetStateAction<GameState>>,
    result: MakeMoveResponse,
): boolean {
    setGame((prevGame) => ({
        ...prevGame,
        fen: result.fen,
        status: result.status,
        winner:
            result.status.type === "checkmate"
                ? result.status.winner
                : undefined,
        evaluation: result.eval,
    }));
    return result.move_was_legal;
}

export async function setFenHandler(
    fen: FenRequest,
    setGame: React.Dispatch<React.SetStateAction<GameState>>
) {
    try {
        const response = await axios.post<GameStateResponse>(
            "/api/from_fen",
            fen
        );

        const result = response.data;

        setGame({
            fen: result.fen,
            winner: undefined,
            status: result.status,
            evaluation: 0.0, // not yet determined
        });
    } catch (error) {
        // Invalid FEN or another request error: leave the game state unchanged.
        alert("Invalid FEN provided");
        return;
    }
}

export async function resetGame(setGame: React.Dispatch<React.SetStateAction<GameState>>) {
    const response = await axios.get<GameStateResponse> (
        "/api/new"
    );

    let result = response.data;

    setGame({
        fen: result.fen,
        winner: undefined,
        status: result.status,
        evaluation: 0.0, // not yet determined
    });
}

export async function undoMoveHandler(setGame: React.Dispatch<React.SetStateAction<GameState>>) {
    const response = await axios.post<string> (
        "/api/undo"
    );
    
    let fen = response.data; // fen or just an error
    setGame((prevGame) => ({ ...prevGame, fen }));
}

export async function makeMove(move: MoveRequest, setGame: React.Dispatch<React.SetStateAction<GameState>>): Promise<boolean> {
  const response = await axios.post<MakeMoveResponse>(
    "/api/move",
    move
  );
  return updateGameState(setGame, response.data);
}

export async function makeComptuerMove(setGame: React.Dispatch<React.SetStateAction<GameState>>): Promise<boolean> {
    const response = await axios.post<MakeMoveResponse> (
        "/api/move_computer"
    );
    return updateGameState(setGame, response.data);
}