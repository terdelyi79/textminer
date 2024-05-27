use tokio::io::AsyncWrite;

use crate::{error::Error, text_extractor::OutputFormat};
use super::utf8_writer::Utf8Writer;

pub struct JsonWriter<'a, W: AsyncWrite + Unpin + Send> {
    utf8_writer: Utf8Writer<'a, W>,
    line_contains_whitespace_only: bool,
    new_separator: Option<String>,
    last_caharcter_was_whitespace: bool,
    whitespace_to_write: Option<char>,
    output_format: OutputFormat
}

impl<'a, W: AsyncWrite + Unpin + Send> JsonWriter<'a, W> {
    pub fn new(utf8_writer: Utf8Writer<'a, W>, output_format: OutputFormat) -> Self {
        Self {
            utf8_writer,
            line_contains_whitespace_only: true,
            new_separator: None,
            last_caharcter_was_whitespace: true,
            whitespace_to_write: None,
            output_format,
        }
    }

    pub async fn start(&mut self) -> Result<(), Error> {
        self.utf8_writer
            .write_string("{\n \"results\": [\n  {\n   \"text\": \"")
            .await
    }

    #[inline]
    pub async fn write_char(&mut self, ch: char) -> Result<(), Error> {
        if !ch.is_whitespace() {
            self.line_contains_whitespace_only = false;
            if let Some(whitespace) = self.whitespace_to_write {
                if whitespace == '\n' {
                    self.utf8_writer.write_string("\\n").await?;
                } else {
                    self.utf8_writer.write_char(whitespace).await?;
                }

                self.whitespace_to_write = None;
            }
            self.last_caharcter_was_whitespace = false;
        } else {
            if self.line_contains_whitespace_only || self.last_caharcter_was_whitespace {
                return Ok(());
            }
            if !self.last_caharcter_was_whitespace {
                self.last_caharcter_was_whitespace = true;

                self.whitespace_to_write = Some(ch);
                return Ok(());
            }

            self.last_caharcter_was_whitespace = true;
        }

        if self.new_separator.is_some() {
            if self.output_format == OutputFormat::Advanced {
                let separator = self.new_separator.as_ref().unwrap();
                self.utf8_writer.write_string("\"\n  },\n  {\n").await?;
                if !separator.is_empty() {
                    self.utf8_writer
                        .write_string(&format!("   \"separator\": \"{}\",\n", separator))
                        .await?;
                }
                self.utf8_writer.write_string("   \"text\": \"").await?;
            } else {
                self.utf8_writer.write_string("\\n").await?;
            }

            self.new_separator = None;
        }

        match ch {
            '"' => self.utf8_writer.write_string("\\\"").await,
            '\\' => self.utf8_writer.write_string("\\\\").await,
            '\n' => self.utf8_writer.write_string("\\n").await,
            _ => self.utf8_writer.write_char(ch).await,
        }
    }

    pub async fn add_break(&mut self, separator: &str) -> Result<(), Error> {
        self.whitespace_to_write = None;
        self.last_caharcter_was_whitespace = true;
        self.new_separator = Some(String::from(separator));
        self.line_contains_whitespace_only = true;
        Ok(())
    }

    pub async fn end(&mut self, error_message: &str) -> Result<(), Error> {
        self.utf8_writer.write_string("\"\n  }\n").await?;
        self.utf8_writer.write_string(" ]").await?;
        if !error_message.is_empty() {
            self.utf8_writer
                .write_string(&format!(",\n \"error\": \"{}\"", error_message))
                .await?;
        }
        self.utf8_writer.write_string("\n}").await?;
        Ok(())
    }
}
