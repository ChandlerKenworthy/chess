import { useState } from "react";
import { Chessboard, type PieceDropHandlerArgs } from "react-chessboard";
import axios from "axios";
import { makeComptuerMove, undoMoveHandler } from "./GameHandlers";

interface ChessGame {
    fen: string
}

const startGameState = {
    fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
} as ChessGame;

interface MoveRequest {
  from: string;
  to: string;
}

async function makeMove(move: MoveRequest): Promise<string> {
  const response = await axios.post<string>(
    "/api/move",
    move
  );

  return response.data;
}

export default function ChessGame() {
    const [game, setGame] = useState<ChessGame>(startGameState);

    function resetGame() {
        // setGame(startGameState);
        // Also needs to update the backend...
        console.log("Not implemented yet");
    }

    async function onPieceDrop({
        piece, sourceSquare, targetSquare
    }: PieceDropHandlerArgs): Promise<boolean> {
        try {
            const fen = await makeMove({
                from: sourceSquare,
                to: targetSquare,
            });
            setGame((prevGame) => ({ ...prevGame, fen }));
        } catch (error) {
            if (axios.isAxiosError(error)) {
                console.error(
                "Move request failed:",
                error.response?.status,
                error.message
                );
            } else {
                console.error(error);
            }
        }

        return true;
    }

    return (
        <div className="mx-auto w-full max-w-xl space-y-4">
            <Chessboard
                options={{
                    position: game.fen,
                    onPieceDrop,
                    allowDragging: true
                }}
            />
            <div className="flex gap-3">
                <button
                onClick={resetGame}
                className="rounded-lg bg-slate-800 px-4 py-2
                            text-white hover:bg-slate-700"
                >
                New game
                </button>

                <button
                onClick={() => undoMoveHandler(setGame)}
                className="rounded-lg border px-4 py-2
                            hover:bg-slate-100"
                >
                Undo
                </button>
                <button
                    onClick={() => makeComptuerMove(setGame)}
                    className="rounded-lg border px-4 py-2
                            hover:bg-slate-100"
                >
                    Computer Move
                </button>
            </div>
        </div>
    );
}