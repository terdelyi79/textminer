use crate::error::Error;
use tokio::io::{AsyncRead, AsyncWrite};

pub trait TextExtractor {
    async fn extract<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send>(
        &mut self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<(), Error>
    where
        Self: Send;
}
