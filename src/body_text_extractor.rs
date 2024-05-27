use axum::body::Body;
use futures::TryStreamExt;
use std::io::ErrorKind;
use tokio_util::io::{ReaderStream, StreamReader};
use crate::{html::html_text_extractor::HtmlTextExtractor, text_extractor::{OutputFormat, TextExtractor}};

// Implements text extraction from a Body object to other one. (Both HTTP request and response contains bodies.)
pub struct BodyTextExtractor {}

impl BodyTextExtractor {

    // Extracts text from a body object and returns it in another body object
    pub async fn extract(request_body: Body, output_format: OutputFormat) -> Body    
    {
        let stream = request_body.into_data_stream();

        // A duplex stream is needed. We write the extracted text to the input and it response body reads from the output
        let (mut input, output) = tokio::io::duplex(1024);

        // Text extraction and returning the response must happen in parallel
        tokio::spawn(async move {
            let mut reader =
                StreamReader::new(stream.map_err(|e| std::io::Error::new(ErrorKind::Other, e)));            
            let result = HtmlTextExtractor {}.extract(&mut reader, &mut input, output_format).await;            

            if let Err(error) = result {
                println!("An error occured while pocessing the reuqest: {}", error.message);
            }
        });
        
        // Response body will read the extracted text from the output stream to return in the response
        Body::from_stream(ReaderStream::new(output))
    }
}
