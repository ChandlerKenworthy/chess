mod error;
mod routes;
mod state;
mod requests;

use std::sync::Arc;
use salvo::prelude::*;
use tokio::sync::Mutex;
use crate::state::{AppState, SharedState};

#[tokio::main]
async fn main() {
    let state: SharedState = Arc::new(
        Mutex::new(AppState::new())
    );

    let router = routes::router(state);
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    Server::new(acceptor).serve(router).await;
}