use crate::error::Error;
use tokio::io::{AsyncRead, AsyncReadExt};

// Object to read caharcters from an UTF8 encoded by stream
pub struct Utf8Reader<'a, R: AsyncRead + Unpin + Send> {
    input: &'a mut R,
}

impl<'a, R: AsyncRead + Unpin + Send> Utf8Reader<'a, R> {
    pub fn new(input: &'a mut R) -> Self {
        Self { input }
    }

    // Read a byte from the stream
    #[inline]
    pub async fn read_byte(&mut self) -> Result<u8, Error> {
        Ok(self.input.read_u8().await?)
    }

    // Read a character from the stream
    #[inline]
    pub async fn read_char(&mut self) -> Result<char, Error> {
        // Decode the next character according to the UTF-8 specification
        let b = self.read_byte().await?;
        let code = match b {
            // Character is encoded is one byte
            0x00..=0x7f => u32::from(b),
            // First character of a unicode character cannot be in this range
            0x80..=0xbf => 0x20, // Change to return error instead
            // Character is encoded in two bytes
            0xc0..=0xdf => u32::from(b & 0x1f) << 6 | u32::from(self.read_byte().await? & 0x3f),
            // Character is encoded in three bytes
            0xe0..=0xef => {
                u32::from(b & 0x0f) << 12
                    | u32::from(self.read_byte().await? & 0x3f) << 6
                    | u32::from(self.read_byte().await? & 0x3f)
            }
            // Character is encoded in four bytes
            _ => {
                u32::from(b & 0x07) << 18
                    | u32::from(self.read_byte().await? & 0x3f) << 12
                    | u32::from(self.read_byte().await? & 0x3f) << 6
                    | u32::from(self.read_byte().await? & 0x3f)
            }
        };

        // Return the character. (Return space (' ') for characters can't be displayed)
        Ok(char::from_u32(code).unwrap_or(' '))
    }
}
