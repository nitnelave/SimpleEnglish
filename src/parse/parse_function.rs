use super::definitions::*;

fn parse_function_name(lexer: &mut Lexer<'_>) -> Result<String, ParseError> {
    let mut name = "To".to_owned();
    while let Some(token) = lexer.next() {
        match token {
            Token::Word => name = name + " " + lexer.slice(),
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
    if lexer.next() != Some(Token::Whitespace) {
        return Err(ParseError(
            "Function body should start with indentation".to_string(),
        ));
    }
    match lexer.next() {
        Some(Token::Word) => {
            if lexer.slice() == "Display" {
                match lexer.next() {
                    Some(Token::String) => Ok(vec![Statement::Display(
                        lexer.slice().trim_matches('"').to_owned(),
                    )]),
                    _ => Err(ParseError("Expected a string after display".to_string())),
                }
            } else {
                Err(ParseError("Unexpected word in statement".to_string()))
            }
        }
        _ => Err(ParseError("Expected display in function body".to_string())),
    }
}

pub fn parse_function_definition(lexer: &mut Lexer<'_>) -> Result<Node, ParseError> {
    let name = parse_function_name(lexer)?;
    if lexer.next() != Some(Token::Newline) {
        return Err(ParseError(
            "Function declaration should end with a newline".to_string(),
        ));
    }
    let body = parse_function_body(lexer)?;
    Ok(Node::FunctionDefinition(FunctionDefinition { name, body }))
}
