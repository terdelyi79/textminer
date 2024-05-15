mod eof;
mod html;
mod text_extractor;
mod body_text_extractor;
mod utf8_reader;
mod utf8_writer;
pub mod error;

use axum::{body::Body, routing::post, Router};
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

async fn extract(request_body: Body) -> Body {
    BodyTextExtractor::extract(request_body).await
    // let mut html_extractor = HtmlTextExtractor {};
    // html_extractor.process(request_body).await    
}
