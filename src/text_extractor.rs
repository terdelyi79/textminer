use std::io::ErrorKind;
use axum::body::Body;
use futures::TryStreamExt;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::io::{ReaderStream, StreamReader};

use crate::error::Error;

pub trait TextExtractor {
    async fn extract<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send>(
        &mut self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<(), Error> where Self: Send;

    // async fn process(mut self: Box<Self>, request_body: Body) -> Body  where Self: Send
    // {
    //     let stream = request_body.into_data_stream();
    //     // let mut reader =
    //     // StreamReader::new(stream.map_err(|e| std::io::Error::new(ErrorKind::Other, e)));
        
    //     let (mut input, mut output) = tokio::io::duplex(1024);

    //     tokio::spawn(async move {
            
            
    //         let mut reader =
    //         StreamReader::new(stream.map_err(|e| std::io::Error::new(ErrorKind::Other, e)));
    //         let x = &mut reader;
    //         let y = &mut input;
    //         self.extract(x, y).await;
    //     });

    //     let response_stream = ReaderStream::new(output);
    //     let response_body = Body::from_stream(response_stream);
            
    //     //self.extract(&mut reader, &mut input).await;
    //     response_body
    // }
}