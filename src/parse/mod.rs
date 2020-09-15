pub mod definitions;
mod parse_function;

pub use definitions::*;
use logos::Logos; // to call Token::lexer.

pub fn parse(input: &str) -> Result<Vec<Node>, ParseError> {
    let mut lexer = Token::lexer(input);
    let mut res = Vec::new();
    match lexer.next() {
        Some(Token::Word) => {
            if lexer.slice() == "To" {
                res.push(parse_function::parse_function_definition(&mut lexer)?)
            }
        }
        _ => (),
    }
    Ok(res)
}
