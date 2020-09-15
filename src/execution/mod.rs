use crate::resolution as ast;

#[derive(Debug)]
pub struct ExecutionError(pub String);

fn get_start(node: &ast::Node) -> Option<&ast::FunctionDefinition> {
    if let ast::Node::FunctionDefinition(def) = node {
        if def.name == "To start" {
            return Some(def);
        }
    }
    None
}

fn run_statement(
    statement: &ast::Statement,
    program: &Vec<ast::Node>,
) -> Result<(), ExecutionError> {
    match statement {
        ast::Statement::Display(message) => println!("{}", message),
    }
    Ok(())
}

fn run_body(body: &Vec<ast::Statement>, program: &Vec<ast::Node>) -> Result<(), ExecutionError> {
    body.iter().map(|s| run_statement(s, program)).collect()
}

pub fn execute(program: Vec<ast::Node>) -> Result<(), ExecutionError> {
    let mut start_iter = program.iter().filter_map(get_start);
    let start = start_iter
        .next()
        .ok_or(ExecutionError("I don't know how \"To start:\"".to_owned()))?;
    if let Some(_) = start_iter.next() {
        return Err(ExecutionError(
            "Conflicting instructions for how \"To start:\"".to_owned(),
        ));
    }
    run_body(&start.body, &program)
}
