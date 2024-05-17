use super::parser_context::ParserContext;
use crate::error::Error;
use tokio::io::{AsyncRead, AsyncWrite};

// Processor decides from which elements to include contents in the output and when extra new line characters are needed.
pub struct Processor {}

impl Processor {

    // Method is called when open tag of an lement is found
    pub async fn on_start_element<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send>(
        element_name: &str,
        context: &mut ParserContext<'_, R, W>,
    ) -> Result<Option<bool>, Error> {
        match element_name {
            // Elements to include in the output
            "body" | "title" => Ok(Some(true)),
            // Elements to exclude from the output
            "script" | "style" => Ok(Some(false)),
            // Elements needing extra new line characters in the output
            "br" | "p" | "div" | "td" | "th" | "li" => {
                context.write('\n').await?;
                Ok(None)
            }
            // Include content of element if content of parent element is included
            _ => Ok(None),
        }
    }

    // Method is called when close tag of an lement is found
    pub async fn on_end_element<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send>(
        element_name: &str,
        context: &mut ParserContext<'_, R, W>,
    ) -> Result<(), Error> {
        match element_name {
            // Elements needing extra new line characters in the output
            "title" | "p" => {
                context.write('\n').await?;
            }
            _ => {}
        }
        Ok(())
    }
}
