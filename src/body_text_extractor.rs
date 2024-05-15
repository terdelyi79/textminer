use axum::body::Body;
use futures::TryStreamExt;
use std::io::ErrorKind;
use tokio_util::io::{ReaderStream, StreamReader};

use crate::{html::html_text_extractor::HtmlTextExtractor, text_extractor::TextExtractor};
pub struct BodyTextExtractor {}

impl BodyTextExtractor {
    pub async fn extract(request_body: Body) -> Body
    where
        Self: Send,
    {
        let stream = request_body.into_data_stream();

        let (mut input, output) = tokio::io::duplex(1024);

        tokio::spawn(async move {
            let mut reader =
                StreamReader::new(stream.map_err(|e| std::io::Error::new(ErrorKind::Other, e)));
            let x = &mut reader;
            let y = &mut input;
            let mut html_text_extractor = HtmlTextExtractor {};
            html_text_extractor.extract(x, y).await;
        });

        let response_stream = ReaderStream::new(output);
        Body::from_stream(response_stream)
    }
}
