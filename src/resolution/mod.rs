pub mod ast;
mod resolve_function;
use crate::parse;

pub use ast::*;

fn resolve_node(raw_node: parse::Node) -> Result<Node, ResolutionError> {
    match raw_node {
        parse::Node::FunctionDefinition(d) => Ok(Node::FunctionDefinition(
            resolve_function::resolve_function_definition(d)?,
        )),
    }
}

pub fn resolve(raw_tree: Vec<parse::Node>) -> Result<Vec<Node>, ResolutionError> {
    raw_tree.into_iter().map(resolve_node).collect()
}
