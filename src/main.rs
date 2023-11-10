use crate::model::ModelController;

pub use self::error::{Error, Result};

use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{middleware, Router};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod ctx;
mod error;
mod model;
mod web;
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
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;

    // let routes_apis = web::routes_tickets::routes(mc.clone())
    // .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(route_static());
    let address = SocketAddr::from(([127, 0, 0, 1], 9090));
    println!("\n--->> Starting server on http://{address}\n");
    axum::Server::bind(&address)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
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

async fn main_response_mapper(res: Response) -> Response {
    println!("--->> {:<12} main_response_mapper", "RESPONSE_MAPPER");

    println!();
    res
}
