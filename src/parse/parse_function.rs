use super::definitions::*;
use super::lexer::*;
use log::trace;

fn parse_function_name(lexer: &mut Lexer<'_>) -> Result<String, ParseError> {
    let mut name = String::from("To");
    loop {
        match lexer.next() {
            Some(Token::Word(word)) => name = name + " " + word,
            Some(Token::Colon) => break,
            Some(token) => {
                return Err(ParseError(format!(
                    "Unexpected {:?}, expected Colon",
                    token
                )));
            }
            None => {
                return Err(ParseError(String::from(
                    "Unexpected end of file, expected Colon",
                )));
            }
        }
    }
    Ok(name)
}

fn parse_function_call(lexer: &mut Lexer<'_>) -> Result<FunctionCall, ParseError> {
    assert!(std::matches!(lexer.peek(), Some(Token::Word(_))));
    let mut res = Vec::new();
    loop {
        match lexer.peek() {
            Some(Token::Dot) => return Ok(FunctionCall { terms: res }),
            Some(Token::Word(w)) => res.push(SimpleValue::Word(String::from(*w))),
            Some(Token::Integer(i)) => res.push(SimpleValue::Integer(*i)),
            Some(Token::String(s)) => res.push(SimpleValue::String(String::from(*s))),
            Some(token) => {
                return Err(ParseError(format!(
                    "Unexpected {:?} while reading a sentence",
                    token
                )));
            }
            None => {
                return Err(ParseError(String::from(
                    "Unexpected end of file while reading a sentence",
                )));
            }
        }
        lexer.next();
    }
}

fn parse_function_body(lexer: &mut Lexer<'_>) -> Result<Vec<Statement>, ParseError> {
    let mut res = Vec::new();
    loop {
        trace!("Parsing statement");
        expect_token(
            lexer,
            Token::Whitespace(0),
            "Function body should start with indentation".to_string(),
        )?;
        match lexer.peek() {
            Some(Token::Word("Display")) => {
                lexer.next();
                if let Token::String(s) = expect_token(
                    lexer,
                    Token::String(""),
                    String::from(r#"Expected a string after "Display""#),
                )? {
                    res.push(Statement::Display(s.trim_matches('"').to_owned()))
                } else {
                    panic!("expect_token returned non-matching token")
                }
            }
            Some(Token::Word(_)) => {
                res.push(Statement::FunctionCall(parse_function_call(lexer)?));
            }
            _ => return Err(ParseError("Expected display in function body".to_string())),
        }
        expect_token(
            lexer,
            Token::Dot,
            String::from("Sentences should end with a period"),
        )?;
        expect_token(
            lexer,
            Token::Newline,
            String::from("Sentences should end with a newline after the period"),
        )?;
        match lexer.peek() {
            Some(Token::Newline) | None => break,
            _ => (),
        }
    }
    Ok(res)
}

pub fn parse_function_definition(lexer: &mut Lexer<'_>) -> Result<Node, ParseError> {
    let name = parse_function_name(lexer)?;
    trace!(r#"Parsing function declaration for "{}""#, &name);
    expect_token(
        lexer,
        Token::Newline,
        "Function declaration should end with a newline".to_string(),
    )?;
    let body = parse_function_body(lexer)?;
    trace!(r#"Parsed function body for "{}""#, &name);
    Ok(Node::FunctionDefinition(FunctionDefinition { name, body }))
}
