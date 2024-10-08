use std::sync::atomic::{AtomicI32, Ordering};

use axum::{http::StatusCode, response::Html, Router};
use tower_http::services::ServeDir;
use askama_axum::{IntoResponse, Template};
use lazy_static::lazy_static;

use crate::{templates, templates::*};

pub async fn index() -> impl IntoResponse
{
    let readme_raw = IndexTemplate::get_readme();
    let readme = markdown::to_html(&readme_raw);
    let template = templates::IndexTemplate {
        title: "Axhat Stack Template",
        readme: &readme,
    };
    let reply_html = template.render().expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

pub fn serve_static_files() -> Router
{
    Router::new().nest_service("/", ServeDir::new("public"))
}

lazy_static! {
    static ref COUNTER: AtomicI32 = AtomicI32::new(1);
}

pub async fn more_content() -> impl IntoResponse
{
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let reply_html = MoreContentTemplate { n }
        .render()
        .expect("Failed to render template");
    (StatusCode::OK, Html(reply_html).into_response())
}

pub async fn error_handler() -> impl IntoResponse
{
    let template = ErrorTemplate {
        status: 404,
        message: "Not found".to_owned(),
    };
    let reply_html = template.render().expect("Failed to render template");
    (StatusCode::NOT_FOUND, Html(reply_html).into_response())
}
