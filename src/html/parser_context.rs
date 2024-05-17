use crate::{error::Error, utf8_reader::Utf8Reader, utf8_writer::Utf8Writer};
use tokio::io::{AsyncRead, AsyncWrite};

pub struct ParserContext<'a, R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send> {
    pub input: &'a mut Utf8Reader<'a, R>,
    output: &'a mut Utf8Writer<'a, W>,
    pub buffer: String,
    pub output_enabled: bool,
    line_contains_whitespace_only: bool,
    last_character_was_whitespace: bool,
    whitespace_to_write: Option<char>,
}

impl<'a, R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send> ParserContext<'a, R, W> {
    pub fn new(
        input: &'a mut Utf8Reader<'a, R>,
        output: &'a mut Utf8Writer<'a, W>,
        buffer: String,
    ) -> Self {
        Self {
            input,
            output,
            buffer,
            output_enabled: false,
            line_contains_whitespace_only: true,
            last_character_was_whitespace: true,
            whitespace_to_write: None,
        }
    }

    #[inline]
    pub async fn write(&mut self, ch: char) -> Result<(), Error> {
        // Content is written to the output only if it is enabled for the current element
        if self.output_enabled {
            // Newline character is written only if the current row contain non-whitespace characters to avoid empty lines
            if ch == '\n' {
                if !self.line_contains_whitespace_only {
                    self.output.write_char('\n').await?;
                }
                self.line_contains_whitespace_only = true;
                self.last_character_was_whitespace = true;
                self.whitespace_to_write = None;
            // Non-whitespace characters
            } else if !ch.is_whitespace() {
                // Whitespaces are delayed to not write any whitespaces at the end of the lines
                if let Some(whitespace) = self.whitespace_to_write {
                    self.output.write_char(whitespace).await?;
                    self.whitespace_to_write = None;
                }
                self.output.write_char(ch).await?;
                self.line_contains_whitespace_only = false;
                self.last_character_was_whitespace = false;
            // Whitespace characters (except of newline)
            } else {
                // Write only one whitespace between the words
                if !self.last_character_was_whitespace {
                    if let Some(whitespace) = self.whitespace_to_write {
                        self.output.write_char(whitespace).await?;
                        self.whitespace_to_write = None;
                    }
                    self.whitespace_to_write = Some(ch);
                }
                self.last_character_was_whitespace = true;
            }
        }

        Ok(())
    }
}
