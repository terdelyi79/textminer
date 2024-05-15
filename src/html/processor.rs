use tokio::io::{AsyncRead, AsyncWrite};
use crate::error::Error;

use super::parser_context::ParserContext;

pub struct Processor {}

impl Processor {
    pub async fn on_start_element<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send>(
        element_name: &str,
        context: &mut ParserContext<'_, R, W>
    ) -> Result<Option<bool>, Error> {        

        match element_name {
            "body" => Ok(Some(true)),
            "title" => Ok(Some(true)),
            "script" => Ok(Some(false)),
            "style" => Ok(Some(false)),                        
            "br" | "p" | "div" | "td" | "th" | "li" => {
                context.write('\n').await?;                
                Ok(None)
            }            
            _ => Ok(None)
        }
    }

    pub async fn on_end_element<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send>(
        element_name: &str,
        context: &mut ParserContext<'_, R, W>
    )  -> Result<(), Error> {

        match element_name {
            "title" | "p" => {
                context.write('\n').await?;
            }
                _ => {}            
        }
        Ok(())
    }
}
