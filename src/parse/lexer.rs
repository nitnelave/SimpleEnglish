use super::definitions::*;
use logos::Logos; // to call Token::lexer.

pub type Lexer<'a> = std::iter::Peekable<logos::Lexer<'a, Token<'a>>>;

pub fn new_lexer(input: &'_ str) -> Lexer<'_> {
    Token::lexer(input).peekable()
}

/// Peek at the next token in the stream. If it is the expected token, consume it. Otherwise,
/// return an error with the given message.
pub fn expect_token(
    lexer: &mut Lexer<'_>,
    tok: Token<'_>,
    message: String,
) -> Result<(), ParseError> {
    if lexer.peek() != Some(&tok) {
        Err(ParseError(message))
    } else {
        lexer.next();
        Ok(())
    }
}
