use super::{
    comment_parser::CommentParser, element_parser::ElementParser, entity_parser::EntityParser,
};
use crate::{error::Error, html::parser_context::ParserContext};
use async_recursion::async_recursion;
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::context;

// Parse the element contents
pub struct ContentParser {}

impl ContentParser {
    #[async_recursion]
    #[allow(clippy::multiple_bound_locations)]
    // Method is called when open tag of an element was processed. (It is called at the root level of document as well)
    pub async fn parse<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send>(
        context: &mut ParserContext<'_, R, W>,
    ) -> Result<Option<String>, Error> {
        loop {
            match context.input.read_char().await? {
                // If the received character is '<', then it can be either a start/end tag of an element or a comment
                '<' => {
                    context.buffer.push('<');

                    // Read some additional characters to decide whether it is a comment or not
                    let mut is_comment = false;
                    let mut ch = context.input.read_char().await?;
                    context.buffer.push(ch);
                    if ch == '!' {
                        ch = context.input.read_char().await?;
                        context.buffer.push(ch);
                        if ch == '-' {
                            ch = context.input.read_char().await?;
                            context.buffer.push(ch);
                            if ch == '-' {
                                is_comment = true;
                            }
                        }
                    }

                    if is_comment {
                        // If we detected the beginning of a comment then use the comment parser to read it
                        CommentParser::parse(context).await?;
                    } else {
                        // Otherwise it must be an element, therefore use the element parser to read it
                        let element_name = ElementParser::parse(context).await?;
                        
                        // Buffer is no longer needed when element is parsed
                        context.buffer.clear();

                        // Returning back while open tag is not found for the received close tag
                        if element_name.is_some() {
                            return Ok(element_name);
                        }
                    }
                }
                // If the received character is '&', then it must be the begining of an entity
                '&' if context.output_enabled => {
                    // Use the netity parser to read the entity
                    EntityParser::parse(context).await?;
                    context.buffer.clear();
                }
                ch =>                 
                {                    
                    // Whitespaces in HTML source are usually not displayed on the page except one space between words.                    
                    if ch.is_whitespace() {                        
                        context.write(' ').await?;
                    } else {
                        context.write(ch).await?;
                    }
                }
            }
        }
    }
}
