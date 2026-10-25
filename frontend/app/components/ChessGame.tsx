import { useState } from "react";
import { Chessboard, type PieceDropHandlerArgs } from "react-chessboard";
import axios from "axios";
import { makeComptuerMove, makeMove, resetGame, undoMoveHandler } from "./GameHandlers";
import { startGameState, type GameState } from "~/types";



export default function ChessGame() {
    const [game, setGame] = useState<GameState>(startGameState);
    
    const alert_illegal_move = () => {
        alert("That move was illegal, please try again.")
    }

    async function onPieceDrop({
        piece, sourceSquare, targetSquare
    }: PieceDropHandlerArgs): Promise<boolean> {
        try {
            const was_legal_move = await makeMove({
                from: sourceSquare,
                to: targetSquare!,
            }, setGame);
            if (!was_legal_move) {
                alert_illegal_move();
            }
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
            <div className="flex align-center justify-center gap-3">
                <p className={
                    game.status.type == "ongoing" ? "text-green-600" :
                    game.status.type == "stalemate" ? "text-zinc-500" :
                    "text-red-600"
                }>{game.status.type}</p>
            </div>
            <Chessboard
                options={{
                    position: game.fen,
                    onPieceDrop,
                    allowDragging: true
                }}
            />
            <div className="flex gap-3">
                <button
                onClick={() => resetGame(setGame)}
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