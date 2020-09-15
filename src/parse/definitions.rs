use logos::Logos;

#[derive(Debug, Logos, PartialEq, Eq)]
pub enum Token {
    #[regex("\"([^\"\\\\]*(\\\\\")*)*\"")]
    String,
    #[regex("[a-zA-Z][a-zA-Z0-9]*")]
    Word,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token("\n")]
    Newline,
    #[regex("  +")]
    Whitespace,
    #[token(" ", logos::skip)]
    #[error]
    Error,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Display(String),
}

#[derive(Debug)]
pub enum Node {
    FunctionDefinition(FunctionDefinition),
}

#[derive(Debug)]
pub struct ParseError(pub String);

pub type Lexer<'a> = logos::Lexer<'a, Token>;
