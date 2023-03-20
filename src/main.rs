// use lcu::client::RiotClient;
//
// use axum::{
//     Router,
//     routing::get,
//     handler::Handler,
//     response::IntoResponse,
//
// };
//
// #[tokio::main]
// async fn main() {
//     let rc = RiotClient::new();
//
//     let app = Router::new()
//         .route("/health", get(|| async { "200" }))
//
// }
//
// async fn fallback(uri: Uri) -> (StatusCode, String) {
//     (StatusCode::NOT_FOUND, format!("No route for {}", uri))
// }

use axum::{
    Server, handler::Handler, http::{Uri, Method}, response::IntoResponse,
    handler::HandlerWithoutStateExt,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // run it with hyper on localhost:3000
    Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(handler.into_make_service())
        .await.unwrap();
}

async fn handler(method: Method, uri: Uri, body: String) -> impl IntoResponse {
    format!("received `{} {}` with body `{:?}`", method, uri, body)
}
