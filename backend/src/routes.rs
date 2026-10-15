use chess_engine::generator::Move;
use salvo::prelude::*;
use salvo::affix_state;
use crate::requests::MoveRequest;
use crate::state::SharedState;

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
    let is_legal = state.generator.get_move_is_legal(&mut mv); // generates the legal moves inside here
    if !is_legal { 
        // Just reject to make the move, and return the current FEN
        return format!("{:?}", state.generator.b.get_fen());
    }

    let undo = state.generator.b.make_move(mv);

    format!("{:?}", state.generator.b.get_fen())
}

#[handler]
async fn get_board(depot: &mut Depot) -> String {
    let state = depot
        .get_typed::<SharedState>()
        .expect("AppState should be injected");

    let state = state.lock().await;

    // TODO: JSON this all up eventually
    format!("{:?}", state.board.get_fen())
}

#[handler]
async fn health() -> &'static str {
    "OK"
}