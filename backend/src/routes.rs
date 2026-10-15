use salvo::prelude::*;
use salvo::affix_state;
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