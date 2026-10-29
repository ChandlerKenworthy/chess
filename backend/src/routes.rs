use chess_engine::engine::game_status::GameStatus;
use chess_engine::generator::Color;
use chess_engine::generator::Move;
use salvo::prelude::*;
use salvo::affix_state;
use crate::requests::GameStateResponse;
use crate::requests::LoadFenRequest;
use crate::requests::MakeMoveResponse;
use crate::requests::MoveRequest;
use crate::state::SharedState;

const DEPTH: i32 = 5;

pub fn router(state: SharedState) -> Router {
    Router::new()
        .hoop(affix_state::inject(state))
        .push(
            Router::with_path("health")
                .get(health)
        )
        .push(
            Router::with_path("board")
            .get(get_board)
        )
        .push(
            Router::with_path("new")
            .get(start_new_game)
        )
        .push(
            Router::with_path("move")
            .post(make_move)
        )
        .push(
            Router::with_path("move_computer")
            .post(make_computer_move)
        )
        .push(
            Router::with_path("undo")
            .post(undo_move)
        )
        .push(
            Router::with_path("from_fen")
            .post(load_fen)
        )
}

#[handler]
async fn load_fen(
    depot: &mut Depot, 
    req: &mut Request
) -> Result<Json<GameStateResponse>, StatusError> {
    let state = depot
        .get_typed::<SharedState>()
        .expect("AppState should be injected");
    let mut state = state.lock().await;

    let fen_request: LoadFenRequest = req.parse_json().await.expect("Failed to parse JSON");

    let game_state = state.from_fen(fen_request.fen);

    match game_state {
        Ok(g) => Ok(Json(g)),
        Err(_) => Err(StatusError::bad_request())
    }
}

#[handler]
async fn start_new_game(depot: &mut Depot) -> Json<GameStateResponse> {
    let state = depot
        .get_typed::<SharedState>()
        .expect("AppState should be injected");
    let mut state = state.lock().await;

    state.reset();

    Json(GameStateResponse{ 
        fen: state.engine.generator.b.get_fen(), 
        winner: None, 
        status: GameStatus::Ongoing 
    })
}

#[handler]
async fn undo_move(depot: &mut Depot) -> Json<String> {
    let state = depot
        .get_typed::<SharedState>()
        .expect("AppState should be injected");
    let mut state = state.lock().await;

    if let Some(undo_state) = state.undo.take() {
        state.engine.generator.b.unmake_move(undo_state);
    }

    Json(state.engine.generator.b.get_fen())
}

// For any human/computer
// is the game in a terminal state? if so just return that & current fen
// otherwise get the legal moves/check human move is legal
// make the move
// check terminal state <-- this is key as otherwise have to wait to press generate again????
// return

#[handler]
async fn make_computer_move(
    depot: &mut Depot,
) -> Json<MakeMoveResponse> {
    let state = depot
        .get_typed::<SharedState>()
        .expect("AppState should be injected");

    let mut state = state.lock().await;

    // Finds the best move and the current status of the game
    let search = state.engine.get_best_move(DEPTH);

    let (fen, status, move_was_legal) = match search.status {
        GameStatus::Ongoing => {
            let mv = search.best_move.unwrap(); // Dirty, should just return a server error instead of panic!
            let undo = state.engine.generator.b.make_move(mv);
            state.undo = Some(undo);

            // Game status could now have updated
            let new_moves = state.engine.generator.get_moves();
            let new_status = state.engine.get_game_status(&new_moves);

            (
                state.engine.generator.b.get_fen(),
                new_status,
                true
            )
        },
        GameStatus::Stalemate => (
            state.engine.generator.b.get_fen(),
            search.status,
            false // the computer had no legal moves to make
        ),
        GameStatus::Checkmate { winner: _ } => (
            state.engine.generator.b.get_fen(),
            search.status,
            false // computer had no legal moves to make
        )
    };

    // Note that search.score is always positive as it is flipped over per colour
    // so multiply by -1 for black else 1
    // flipped as to_move is not who just made a move
    let sign = match state.engine.generator.b.to_move {
        Color::White => -1.0,
        Color::Black => 1.0
    };
    Json(MakeMoveResponse { fen, status, move_was_legal, eval: sign * search.score })
}

#[handler]
async fn make_move(depot: &mut Depot, req: &mut Request) -> Json<MakeMoveResponse> {
    // Make the move, and return the resulting FEN
    let state = depot
        .get_typed::<SharedState>()
        .expect("AppState should be injected");
    let mut state = state.lock().await;

    let move_request: MoveRequest = req.parse_json().await.expect("Failed to parse JSON");

    // convert to a Move object.
    let mut mv = Move::from_request(
        move_request.from,
        move_request.to,
    );

    // Before checking if this move is legal, generate the legal moves
    // so we can handle terminal position checks first
    let moves = state.engine.generator.get_moves();
    let status = state.engine.get_game_status(&moves);

    // If the game has ended, just return the current board FEN
    // otherwise check the users move is legal, if it is make it and return the new FEN
    // otherwise reject and return the current FEN
    let (fen, status, move_was_legal) = match status {
        GameStatus::Ongoing => {
            // Check if the move is legal, and if it is, update all the flags etc.
            let is_legal = state.engine.generator.get_move_is_legal(&mut mv, &moves);
            if is_legal {
                state.undo = Some(state.engine.generator.b.make_move(mv));
                //  Game status could now have changed after making the move, so check it again
                let new_moves = state.engine.generator.get_moves();
                let new_status = state.engine.get_game_status(&new_moves);

                (
                    state.engine.generator.b.get_fen(),
                    new_status,
                    true
                )
            } else {
                (
                    state.engine.generator.b.get_fen(),
                    status,
                    false
                )
            }
        },
        GameStatus::Stalemate => (
            state.engine.generator.b.get_fen(),
            status,
            false
        ),
        GameStatus::Checkmate { winner: _ } => (
            state.engine.generator.b.get_fen(),
            status,
            false
        )
    };

    Json(MakeMoveResponse { fen, status, move_was_legal, eval: 0.0 })
}

#[handler]
async fn get_board(depot: &mut Depot) -> Json<String> {
    let state = depot
        .get_typed::<SharedState>()
        .expect("AppState should be injected");

    let state = state.lock().await;
    Json(state.engine.generator.b.get_fen())
}

#[handler]
async fn health() -> &'static str {
    "OK"
}