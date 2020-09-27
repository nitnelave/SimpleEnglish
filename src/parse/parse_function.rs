use super::definitions::*;
use super::lexer::*;

fn parse_function_name(lexer: &mut Lexer<'_>) -> Result<String, ParseError> {
    let mut name = "To".to_owned();
    while let Some(token) = lexer.next() {
        match token {
            Token::Word(word) => name = name + " " + word,
            Token::Colon => break,
            _ => {
                return Err(ParseError(format!(
                    "Unexpected {:?}, expected Colon",
                    token
                )));
            }
        }
    }
    Ok(name)
}

fn parse_function_body(lexer: &mut Lexer<'_>) -> Result<Vec<Statement>, ParseError> {
    expect_token(
        lexer,
        Token::Whitespace(2),
        "Function body should start with indentation".to_string(),
    )?;
    match lexer.next() {
        Some(Token::Word("Display")) => match lexer.next() {
            Some(Token::String(string)) => Ok(vec![Statement::Display(
                string.trim_matches('"').to_owned(),
            )]),
            _ => Err(ParseError("Expected a string after display".to_string())),
        },
        Some(Token::Word(word)) => Err(ParseError(format!(
            "Unexpected word in statement: {}",
            word
        ))),
        _ => Err(ParseError("Expected display in function body".to_string())),
    }
}

pub fn parse_function_definition(lexer: &mut Lexer<'_>) -> Result<Node, ParseError> {
    let name = parse_function_name(lexer)?;
    expect_token(
        lexer,
        Token::Newline,
        "Function declaration should end with a newline".to_string(),
    )?;
    let body = parse_function_body(lexer)?;
    Ok(Node::FunctionDefinition(FunctionDefinition { name, body }))
}
