import axios from "axios";

export async function undoMoveHandler(setGame: React.Dispatch<React.SetStateAction<{ fen: string }>>) {
    const response = await axios.post<string> (
        "/api/undo"
    );
    
    let fen = response.data; // fen or just an error
    setGame((prevGame) => ({ ...prevGame, fen }));
}