use crate::error::Error;
use serde::Deserialize;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Deserialize, PartialEq, Clone, Copy)]
pub enum OutputFormat { Simple, Advanced }

#[derive(Deserialize)]
pub struct ExtractParameters {
    pub output_format: OutputFormat
}

pub trait TextExtractor {
    async fn extract<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send>(
        &mut self,
        reader: &mut R,
        writer: &mut W,
        output_format: OutputFormat
    ) -> Result<(), Error>
    where
        Self: Send;
}
