import axios from "axios";
import { type GameStateResponse, type GameState, type MakeMoveResponse, type MoveRequest } from "~/types";

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
    }));
    return result.move_was_legal;
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