use super::parser_context::ParserContext;
use crate::error::Error;
use tokio::io::{AsyncRead, AsyncWrite};

// Parse comments from HTML
pub struct CommentParser {}

impl CommentParser {
    // Parse method is called when comment is already started by the <!-- prefix
    pub async fn parse<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send>(
        context: &mut ParserContext<'_, R, W>,
    ) -> Result<(), Error> {
        loop {
            match context.input.read_char().await? {
                // When the '<' character is received, then it can be the end of the comment
                '>' => {
                    context.buffer.push('>');

                    // End of comment was reached only when the buffer ends with -->
                    if context.buffer.ends_with("-->") {
                        // There isn't anything to do with comments, just clear the buffer and return
                        context.buffer.clear();
                        return Ok(());
                    }
                }
                ch => {
                    // While end of comment is not reached, the whole comment is stored in the buffer
                    context.buffer.push(ch);
                }
            }
        }
    }
}
