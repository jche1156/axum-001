use axum::{routing::get, routing::post, Router};
use tower_http::services::ServeDir;

mod index;
mod rest;

#[tokio::main]
async fn main() {
    let assets_path = std::env::current_dir().unwrap();
    let assets_serve_dir = ServeDir::new(format!(
        "{}/assets",
        assets_path.to_str().expect("Assets path is not utf8")
    ));
    let app = Router::new()
        .route("/", get(index::render))
        .route("/rest/hi", post(rest::htmx_msg))
        .nest_service("/assets", assets_serve_dir);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7890").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
