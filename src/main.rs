use axum::{http::StatusCode, response::Html, routing::get, Router};
use askama_axum::{IntoResponse, Template};
use tower_http::services::ServeDir;
use std::sync::atomic::{AtomicI32, Ordering};
use lazy_static::lazy_static;

#[tokio::main]
async fn main()
{
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/more-content", get(more_content))
        .nest("/public", serve_static_files());

    let listener =
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("Failed to bind port. port 3000 Already in use?");
    println!("Server running on: http://localhost:3000");
        
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct MyTemplate {}

async fn root() -> impl IntoResponse
{
    let template = MyTemplate {};
    let reply_html = template.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

fn serve_static_files() -> Router
{
    Router::new().nest_service("/", ServeDir::new("public"))
}

lazy_static! {
    static ref COUNTER: AtomicI32 = AtomicI32::new(0);
}

#[derive(Template)]
#[template(path = "more-content.html")]
struct MoreContentTemplate {
    n: i32,
}

async fn more_content() -> impl IntoResponse {
    // Increment the counter atomically
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    // Generate the HTML for the new list item
    let reply_html = format!("<li>Content loaded: {}</li>", n);
    (StatusCode::OK, Html(reply_html).into_response())
}
