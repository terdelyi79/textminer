use tokio::io::{AsyncRead, AsyncWrite};
use crate::error::Error;

use super::{content_parser::ContentParser, parser_context::ParserContext, processor::Processor};

pub struct ElementParser {}

impl ElementParser {
    pub async fn parse<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send>(
        context: &mut ParserContext<'_, R, W>,
    ) -> Result<Option<String>, Error>
    {
        let mut name_length = 0;

        loop {
            match context.input.read_char().await? {
                '>' => {
                    context.buffer.push('>');

                    if name_length == 0 {
                        name_length = context.buffer.len();
                    }

                    // Open element
                    if !context.buffer.starts_with("</") && !context.buffer.ends_with("/>") {
                        let element_name =
                            String::from(&context.buffer[1..name_length - 1]).to_lowercase();

                        let old_output_enabled = context.output_enabled;

                        if let Some(new_output_enabled) = Processor::on_start_element(&element_name, context).await?
                        {
                            context.output_enabled = new_output_enabled;
                        }
                        
                        context.buffer.clear();                        

                        let close_element = ContentParser::parse(context).await?;                        

                        if let Some(val) = close_element {
                            if val == element_name {
                                context.output_enabled = old_output_enabled;
                                return Ok(None);
                            } else {
                                return Ok(Some(val));
                            }
                        }
                    }
                    // Empty element
                    else if context.buffer.ends_with("/>") {                        
                        let mut element_name = String::from(&context.buffer[1..name_length - 1]).to_lowercase();
                        let trimmed = element_name.strip_suffix('/');
                        if let Some(trimmed_value) = trimmed
                        {
                            element_name = trimmed_value.into();
                        }

                        Processor::on_start_element(&element_name, context).await?;
                        Processor::on_end_element(&element_name, context).await?;
                        context.buffer.clear();
                        return Ok(None);
                    }
                    // Close element
                    else {
                        let element_name = String::from(&context.buffer[2..name_length - 1]).to_lowercase();
                        Processor::on_end_element(&element_name, context).await?;
                        context.buffer.clear();
                        return Ok(Some(element_name));
                    }
                },
                ch => {
                    context.buffer.push(ch);
                    if ch.is_whitespace() && name_length == 0 {
                        name_length = context.buffer.len();
                    }
                    
                }
            }
        }
    }
}
