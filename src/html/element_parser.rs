use super::{content_parser::ContentParser, parser_context::ParserContext, processor::Processor};
use crate::error::Error;
use tokio::io::{AsyncRead, AsyncWrite};

// Parse the open and close tags of elements
pub struct ElementParser {}

impl ElementParser {

    // Method is called when an open, close or empty tag of element was detected
    pub async fn parse<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send>(
        context: &mut ParserContext<'_, R, W>,
    ) -> Result<Option<String>, Error> {
        let mut name_length = 0;

        loop {
            match context.input.read_char().await? {
                // If the '>' character was received than the and of the tag was reached
                '>' => {
                    context.buffer.push('>');

                    // Detect the length of element name (without attributes)
                    if name_length == 0 {
                        name_length = context.buffer.len();
                    }

                    // An open element doesn't contain slash either at the beginning or at the end
                    if !context.buffer.starts_with("</") && !context.buffer.ends_with("/>") {
                        let element_name =
                            String::from(&context.buffer[1..name_length - 1]).to_lowercase();
                         
                        if element_name == "!doctype" {
                            return Ok(None);
                        }

                        let old_output_enabled = context.output_enabled;

                        // Processor decided whether to extract content from the element found
                        if let Some(new_output_enabled) =
                            Processor::on_start_element(&element_name, context).await?
                        {
                            context.output_enabled = new_output_enabled;
                        }

                        context.buffer.clear();

                        // Content is used to process element content, what returns only if a close tag was found
                        let close_element = ContentParser::parse(context, false).await?;

                        // The close tag may not match this open tag. (Some tags like <br> has no closing pair.)
                        if let Some(val) = close_element {
                            // If the close tag matches the open tag, then change back the setting whether to extract content
                            if val == element_name {
                                context.output_enabled = old_output_enabled;
                                // None means that open tag for the closing one was already found
                                return Ok(None);
                            } else {
                                // No matching open tag was found for the closing one. It must be checked at higher level as well
                                return Ok(Some(val));
                            }
                        }
                    }
                    // If there is a slash at the end of the name, then it is an XHTML compatible empty element like <br/>
                    else if context.buffer.ends_with("/>") {
                        
                        let mut element_name =
                            String::from(&context.buffer[1..name_length - 1]).to_lowercase();
                        let trimmed = element_name.strip_suffix('/');
                        if let Some(trimmed_value) = trimmed {
                            element_name = trimmed_value.into();
                        }

                        // Processor is called in the same way as for an open and close tag (<br/> is the same as <br></br>)
                        Processor::on_start_element(&element_name, context).await?;
                        Processor::on_end_element(&element_name, context).await?;

                        context.buffer.clear();

                        // No element name is returned, because there is no close tag to find open tag for
                        return Ok(None);
                    }
                    // In all remaining cases it must be a close tag
                    else {
                        let element_name =
                            String::from(&context.buffer[2..name_length - 1]).to_lowercase();                        
                        
                        Processor::on_end_element(&element_name, context).await?;
                        context.buffer.clear();

                        // Return element name to find the matching open tag
                        return Ok(Some(element_name));
                    }
                }
                // All other characters are processed in the same way
                ch => {
                    // Tag content is stored in the buffer, because it contains valuable information like element name and attributes
                    context.buffer.push(ch);
                    
                    // Check for whitespaces to find the end of the element name
                    if ch.is_whitespace() && name_length == 0 {
                        name_length = context.buffer.len();
                    }
                }
            }
        }
    }
}
