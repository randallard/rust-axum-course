use axum::extract::Path;
#[allow(unused)]
use axum::extract::Query;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2));

    // region: --- Start Server

    let addr = SocketAddr::from(([127,0,0,1],8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
    .serve(routes_hello.into_make_service())
    .await
    .unwrap()

    // endregion: --- Start Server
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello2 - {name:?}", "HANDLER");
    Html(format!("Hello2 <strong>{name}</strong>"))
}
