use salvo::prelude::*;

pub fn router() -> Router {
    Router::new()
        .push(Router::with_path("health").get(health))
}

#[handler]
async fn health() -> &'static str {
    "OK"
}