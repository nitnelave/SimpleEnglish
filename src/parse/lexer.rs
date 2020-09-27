use super::definitions::*;
use logos::Logos; // to call Token::lexer.

pub type Lexer<'a> = std::iter::Peekable<logos::Lexer<'a, Token<'a>>>;

pub fn new_lexer(input: &'_ str) -> Lexer<'_> {
    Token::lexer(input).peekable()
}

/// Peek at the next token in the stream. If it is the expected token, consume it. Otherwise,
/// return an error with the given message.
pub fn expect_token<'input>(
    lexer: &mut Lexer<'input>,
    tok: Token<'input>,
    message: String,
) -> Result<Token<'input>, ParseError> {
    match lexer.peek() {
        Some(&token) => {
            if std::mem::discriminant(&token) == std::mem::discriminant(&tok) {
                lexer.next();
                Ok(token)
            } else {
                Err(ParseError(message))
            }
        }
        None => Err(ParseError(String::from("Unexpected end of sentence"))),
    }
}
