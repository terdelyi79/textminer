mod body_text_extractor;
pub mod error;
mod html;
mod text_extractor;
mod utf8_reader;
mod utf8_writer;

use axum::{body::Body, http::StatusCode, response::IntoResponse, routing::post, Router};
use body_text_extractor::BodyTextExtractor;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/extract", post(extract));
    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn extract(request_body: Body) -> impl IntoResponse {
    // The status code 202 (ACCEPTED) indicates that HTTP header is sent before processing is finished, therefore status is not known
    (StatusCode::ACCEPTED, BodyTextExtractor::extract(request_body).await)
}
