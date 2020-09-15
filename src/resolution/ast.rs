#[derive(Debug)]
pub struct ResolutionError(String);

#[derive(Debug)]
pub struct CustomType {
    pub name: String,
    pub contents: Vec<TypedVariable>,
}

#[derive(Debug)]
pub enum Type {
    Integer,
    Number,
    String,
    CustomType(CustomType),
}

#[derive(Debug)]
pub struct TypedVariable {
    pub name: String,
    pub typ: Type,
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub name: String,
    pub arguments: Vec<TypedVariable>,
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
