mod error;
mod routes;
mod state;

use salvo::prelude::*;

#[tokio::main]
async fn main() {

    let router = routes::router();
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;

    Server::new(acceptor).serve(router).await;
}