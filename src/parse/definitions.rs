use logos::Logos;

#[derive(Debug, Logos, PartialEq, Eq, Copy, Clone)]
pub enum Token<'a> {
    #[regex(r#""([^"\\]*(\\")*)*""#, |lex| lex.slice())]
    String(&'a str),
    // A word starts with a letter.
    #[regex(r"[[:alpha:]][-a-zA-Z0-9_']*", |lex| lex.slice())]
    Word(&'a str),
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),
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

#[derive(Debug, PartialEq, Eq)]
pub enum SimpleValue {
    Word(String),
    String(String),
    Integer(i64),
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub terms: Vec<SimpleValue>,
}

#[derive(Debug)]
pub enum Statement {
    Display(String),
    FunctionCall(FunctionCall),
}

#[derive(Debug)]
pub enum Node {
    FunctionDefinition(FunctionDefinition),
}

#[derive(Debug)]
pub struct ParseError(pub String);
