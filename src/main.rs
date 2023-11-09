use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, get_service};
use axum::Router;
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello_2))
}

fn route_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./static")))
}

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .fallback_service(route_static());
    let address = SocketAddr::from(([127, 0, 0, 1], 9090));
    println!("\n--->> Starting server on http://{address}\n");
    axum::Server::bind(&address)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("--->> {:<12} handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!(
        "<p>Welcome to awesome <strong>{name}</strong> rust<p>"
    ))
}

async fn handler_hello_2(Path(name): Path<String>) -> impl IntoResponse {
    println!("--->> {:<12} handler_hello - {name:?}", "HANDLER");

    Html(format!(
        "<p>Welcome to awesome <strong>{name}</strong> rust<p>"
    ))
}
