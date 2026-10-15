import { useState } from "react";
import { Chessboard, type DraggingPieceDataType, type PieceDropHandlerArgs } from "react-chessboard";

interface ChessGame {
    fen: string
}

const startGameState = {
    fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
} as ChessGame;

export default function ChessGame() {
    const [game, setGame] = useState<ChessGame>(startGameState);

    function resetGame() {
        setGame(startGameState);
    }

    function onPieceDrop({
        piece, sourceSquare, targetSquare
    }: PieceDropHandlerArgs): boolean {
        console.log("Source: ", sourceSquare);
        console.log("target:", targetSquare);
        console.log(piece.pieceType)

        // Send the data to the backend and get back either
        // a new FEN or an error, in which case the move was
        // invalid

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
                onClick={() => console.log("implement me")}
                className="rounded-lg border px-4 py-2
                            hover:bg-slate-100"
                >
                Undo
                </button>
            </div>
        </div>
    );
}