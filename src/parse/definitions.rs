use logos::Logos;

#[derive(Debug, Logos, PartialEq, Eq)]
pub enum Token<'a> {
    #[regex(r#""([^"\\]*(\\")*)*""#, |lex| lex.slice())]
    String(&'a str),
    #[regex("[a-zA-Z][a-zA-Z0-9]*", |lex| lex.slice())]
    Word(&'a str),
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token("\n")]
    Newline,
    #[regex("  +", |lex| lex.slice().len())]
    Whitespace(usize),
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
