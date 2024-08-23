use crate::grammar::CommandParser;
#[derive(Debug)]
pub enum ParseError {
    Error(String),
}

pub fn parse(input: &str) -> Result<Vec<String>, ParseError> {
    let parser = CommandParser::new();
    match parser.parse(input) {
        Ok(result) => Ok(result),
        Err(e) => Err(ParseError::Error(format!("Parse error: {:?}", e))),
    }
}
