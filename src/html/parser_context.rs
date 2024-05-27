use crate::{error::Error, util::{json_writer::JsonWriter, utf8_reader::Utf8Reader}};
use tokio::io::{AsyncRead, AsyncWrite};

pub struct ParserContext<'a, R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send> {
    pub input: &'a mut Utf8Reader<'a, R>,
    pub output: &'a mut JsonWriter<'a, W>,
    pub buffer: String,
    pub output_enabled: bool
}

impl<'a, R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send> ParserContext<'a, R, W> {
    pub fn new(
        input: &'a mut Utf8Reader<'a, R>,
        output: &'a mut JsonWriter<'a, W>,
        buffer: String,
    ) -> Self {
        Self {
            input,
            output,
            buffer,
            output_enabled: false
        }
    }

    #[inline]
    pub async fn write(&mut self, ch: char) -> Result<(), Error> {
        if self.output_enabled {
            self.output.write_char(ch).await?;
        }
        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), Error> {    
        self.output.start().await
    }

    pub async fn end(&mut self, message: &str) -> Result<(), Error> {    
        self.output.end(message).await
    }

}
