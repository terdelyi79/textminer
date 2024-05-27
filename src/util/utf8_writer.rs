use crate::error::Error;
use tokio::io::{AsyncWrite, AsyncWriteExt};

// Object to write caharcters to a stream using UTF8 encoding
pub struct Utf8Writer<'a, W: AsyncWrite + Unpin + Send> {
    output: &'a mut W,
    buffer_size: usize,
    buffer_pos: usize,
}

impl<'a, W: AsyncWrite + Unpin + Send> Utf8Writer<'a, W> {
    pub fn new(output: &'a mut W, buffer_size: usize) -> Self {
        Self {
            output,
            buffer_size,
            buffer_pos: 0,
        }
    }

    // Write a byte into the output stream
    #[inline]
    pub async fn write_byte(&mut self, b: u8) -> Result<(), Error> {
        self.output.write_u8(b).await.unwrap();
        
        // Count of the number of bytes arelady buffered in the output stream
        self.buffer_pos += 1;
        
        // If buffer is full then flush it and reset the buffer positions
        if self.buffer_pos == self.buffer_size - 1 {
            self.output.flush().await.unwrap();
            self.buffer_pos = 0;
        }
        
        Ok(())
    }

    // Write a character into the output stream
    #[inline]
    pub async fn write_char(&mut self, ch: char) -> Result<(), Error> {
        let code = ch as u32;
        match code {
            0x00..=0x7f => {
                self.write_byte(code as u8).await?;
            }
            0x80..=0x07ff => {
                let b1 = code & 0x3f | 0x80;
                let b2 = (code >> 6) & 0x1f | 0xC0;
                self.write_byte(b2 as u8).await?;
                self.write_byte(b1 as u8).await?;
            }
            0x0800..=0xffff => {
                let b1 = code & 0x3f | 0x80;
                let b2 = (code >> 6) & 0x3f | 0x80;
                let b3 = (code >> 12) & 0x0f | 0xe0;
                self.write_byte(b3 as u8).await?;
                self.write_byte(b2 as u8).await?;
                self.write_byte(b1 as u8).await?;
            }
            0x10000.. => {
                let b1 = code & 0x3f | 0x80;
                let b2 = (code >> 6) & 0x1f | 0xC0;
                let b3 = (code >> 12) & 0x0f | 0xe0;
                let b4 = (code >> 18) & 0x07 | 0xf0;
                self.write_byte(b4 as u8).await?;
                self.write_byte(b3 as u8).await?;
                self.write_byte(b2 as u8).await?;
                self.write_byte(b1 as u8).await?;
            }
        }

        Ok(())
    }

    #[inline]
    pub async fn write_string(&mut self, s: &str) -> Result<(), Error> {
        for ch in s.chars() {        
            self.write_char(ch).await?;
        }
        Ok(())
    }
}
