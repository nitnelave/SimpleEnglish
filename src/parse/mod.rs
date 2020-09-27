pub mod definitions;
mod lexer;
mod parse_function;

pub use definitions::*;
use lexer::*;

pub fn parse(input: &str) -> Result<Vec<Node>, ParseError> {
    let mut lexer = new_lexer(input);
    let mut res = Vec::new();
    loop {
        match lexer.next() {
            Some(Token::Word("To")) => {
                res.push(parse_function::parse_function_definition(&mut lexer)?)
            }
            Some(Token::Newline) | Some(Token::Whitespace(_)) => (),
            None => break,
            Some(token) => {
                return Err(ParseError(format!(
                    "Unexpected {:?}, expected a function declaration",
                    token
                )));
            }
        }
    }
    Ok(res)
}
