
use std::net::SocketAddr;
use axum::extract::Query;
use serde::Deserialize;
use axum::Router;
use axum::response::{Html, IntoResponse};
use axum::routing::get;

#[derive(Debug, Deserialize)]
struct HelloParams{
    name: Option<String>,
}

#[tokio::main]
async fn main() {

    let routes_hello = Router::new().route(
    "/hello",
    get(handler_hello),
    );

    let address  =  SocketAddr::from(([127, 0, 0, 1], 9090));
    println!("\n--->> Starting server on http://{address}\n");
    axum::Server::bind(&address)
        .serve(routes_hello.into_make_service())
    .await
    .unwrap();


}


async  fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {

    println!("--->> {:<12} handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
 Html(format!("<p>Welcome to awesome <strong>{name}</strong> rust<p>"))

}
