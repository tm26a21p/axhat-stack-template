
use axum::{routing::get, Router};
use crate::routes::*;
mod routes;
mod templates;

#[tokio::main]
async fn main()
{
    tracing_subscriber::fmt::init();
    let address = "0.0.0.0";
    let port = "4004";
    let app = Router::new()
        .route("/", get(index))
        .route("/more-content", get(more_content))
        .nest("/public", routes::serve_static_files());

    let listener = tokio::net::TcpListener::bind(address.to_owned() + ":" + &port)
        .await
        .expect("Failed to bind port.");
    println!("Server running on http://{}:{}", "localhost", port);

    axum::serve(listener, app).await.unwrap();
}
