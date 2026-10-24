use chess_engine::generator::Move;
use salvo::prelude::*;
use salvo::affix_state;
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
}

#[handler]
async fn undo_move(depot: &mut Depot) -> String {
    let state = depot
        .get_typed::<SharedState>()
        .expect("AppState should be injected");
    let mut state = state.lock().await;

    if let Some(undo_state) = state.undo.take() {
        state.engine.generator.b.unmake_move(undo_state);
    }

    format!("{:?}", state.engine.generator.b.get_fen())
}

#[handler]
async fn make_computer_move(depot: &mut Depot) -> String {
    // Get the engine to get the best move
    // Make the move
    // Store the undo state
    // Return the new FEN

    let state = depot
        .get_typed::<SharedState>()
        .expect("AppState should be injected");
    let mut state = state.lock().await;

    let mv = state.engine.get_best_move(DEPTH);
    let undo = state.engine.generator.b.make_move(mv);
    state.undo = Some(undo);

    format!("{:?}", state.engine.generator.b.get_fen())
}

#[handler]
async fn make_move(depot: &mut Depot, req: &mut Request) -> String {
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

    // Check if the move is legal, and if it is, update all the flags etc.
    let is_legal = state.engine.generator.get_move_is_legal(&mut mv); // generates the legal moves inside here
    if !is_legal { 
        // Just reject to make the move, and return the current FEN
        return format!("{:?}", state.engine.generator.b.get_fen());
    }

    state.undo = Some(state.engine.generator.b.make_move(mv));

    format!("{:?}", state.engine.generator.b.get_fen())
}

#[handler]
async fn get_board(depot: &mut Depot) -> String {
    let state = depot
        .get_typed::<SharedState>()
        .expect("AppState should be injected");

    let state = state.lock().await;

    // TODO: JSON this all up eventually
    format!("{:?}", state.engine.generator.b.get_fen())
}

#[handler]
async fn health() -> &'static str {
    "OK"
}