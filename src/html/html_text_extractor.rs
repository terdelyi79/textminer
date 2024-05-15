use tokio::io::{AsyncRead, AsyncWrite};
use crate::{error::Error, text_extractor::TextExtractor, utf8_reader::Utf8Reader, utf8_writer::Utf8Writer};
use super::{content_parser::ContentParser, parser_context::ParserContext};

pub struct HtmlTextExtractor {}

impl TextExtractor for HtmlTextExtractor {
    async fn extract<R: AsyncRead + Unpin + Send, W: AsyncWrite + Unpin + Send>(
        &mut self,
        reader: &mut R,
        writer: &mut W,
    ) -> Result<(), Error> {
        let mut utf8_reader = Utf8Reader::new(reader);
        let mut utf8_writer = Utf8Writer::new(writer, 1024);
        let buffer = String::new();
        let mut context = ParserContext::new(&mut utf8_reader, &mut utf8_writer, buffer);
        ContentParser::parse(&mut context).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {   

    use crate::body_text_extractor::BodyTextExtractor;    
    use axum::body::{to_bytes, Body};    

    #[tokio::test]
    async fn test_basic() {
        do_test("<html><body>Test Content</body></html>", "Test Content").await;
        do_test("<html><body>Test&nbsp;Content</body></html>", "Test Content").await;
        do_test("<HTML><BODY>Test Content</BODY></HTML>", "Test Content").await;
        do_test("<html><body>Test\nContent</body></html>", "Test Content").await;
        do_test("<html><body>Test<br>Content</body></html>", "Test\nContent").await;
        do_test("<html><body>Test<br />Content</body></html>", "Test\nContent").await;
        do_test("<html><body><p>Paragraph 1</p><p>Paragraph 2</p></body></html>", "Paragraph 1\nParagraph 2\n").await;
        do_test("<html><body>First &amp; Second</body></html>", "First & Second").await;
        do_test("<html><body onload=\"load();\">Test Content</body></html>", "Test Content").await;
        do_test("<html><body>Test <!-- Comment -->Content</body></html>", "Test Content").await;
        do_test("<meta name=\"twitter:url\" content=\"https://uplandsoftware.com/\"/>", "").await;
        do_test("<html><body>Test <!-- <img src=\"http://localhost\"> -->Content</body></html>", "Test Content").await;
        do_test("<html><head><title>Title</title><script>Script</script><link rel=\"stylesheet\"></head><body>Body</body></html>", "Title\nBody").await;
        do_test("<html><body><script>if (document.body.addEventListener(\"load\", (t => { t.target.classList.contains(\"interactive\") && t.target.setAttribute(\"data-readystate\", \"complete\") }), { capture: !0 }), window && document.documentElement) { const t = { light: \"#ffffff\", dark: \"#1b1b1b\" }; try { const e = window.localStorage.getItem(\"theme\"); e && (document.documentElement.className = e, document.documentElement.style.backgroundColor = t[e]) } catch (t) { console.warn(\"Unable to read theme from localStorage\", t) } }</script><div id=\"root\">Text</div></body></html>", "Text").await;
    }

    async fn do_test(input: &str, expected_output: &str)
    {
        let request_body = Body::from(String::from(input));        
        //let response_body = HtmlTextExtractor {}.process(request_body).await;
        let response_body = BodyTextExtractor::extract(request_body).await;
        let response_bytes = to_bytes(response_body, usize::MAX).await.unwrap();
        let output = std::str::from_utf8(&response_bytes).unwrap();
        assert_eq!(output, expected_output);
    }
}