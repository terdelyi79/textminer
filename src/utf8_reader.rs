use tokio::io::{AsyncRead, AsyncReadExt};
use crate::error::Error;

pub struct Utf8Reader<'a, R: AsyncRead + Unpin + Send> {
    input: &'a mut R
}

impl<'a, R> Utf8Reader<'a, R> where R: AsyncRead + Unpin + Send {
    pub fn new(input: &'a mut R) -> Self {        
        Self { input }
    }

    #[inline]
    pub async fn read_byte(&mut self) -> Result<u8, Error>
    {
        Ok(self.input.read_u8().await?)
    }

    #[inline]
    pub async fn read_char(&mut self) -> Result<char, Error>
    {        
        // Decode the next character according to the UTF-8 specification
        let b = self.read_byte().await?;
        let code = match b
        {
            0x00..=0x7f => u32::from(b),
            0x80..=0xbf => 0x20, // Change to return error instead
            0xc0..=0xdf => u32::from(b & 0x1f) << 6 | u32::from(self.read_byte().await? & 0x3f),
            0xe0..=0xef => u32::from(b & 0x0f) << 12 | u32::from(self.read_byte().await? & 0x3f) << 6 | u32::from(self.read_byte().await? & 0x3f),
            _ => u32::from(b & 0x07) << 18 | u32::from(self.read_byte().await? & 0x3f) << 12 | u32::from(self.read_byte().await? & 0x3f) << 6 | u32::from(self.read_byte().await? & 0x3f)
        };

        let ch = char::from_u32(code).unwrap_or(' ');        

        // Return the character. (Return space (' ') for characters can't be displayed)
        Ok(ch)
    }
}
