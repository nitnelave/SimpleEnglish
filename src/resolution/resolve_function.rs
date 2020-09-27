use super::ast::*;
use crate::parse;

fn extract_arguments(_function_name: &str) -> Result<Vec<TypedVariable>, ResolutionError> {
    Ok(Vec::new())
}

fn resolve_function_call(
    call: parse::definitions::FunctionCall,
) -> Result<FunctionCall, ResolutionError> {
    Ok(FunctionCall { terms: call.terms })
}

fn resolve_statement(raw_statement: parse::Statement) -> Result<Statement, ResolutionError> {
    match raw_statement {
        parse::Statement::Display(message) => Ok(Statement::Display(message)),
        parse::Statement::FunctionCall(call) => {
            Ok(Statement::FunctionCall(resolve_function_call(call)?))
        }
    }
}

pub fn resolve_function_definition(
    raw_definition: parse::FunctionDefinition,
) -> Result<FunctionDefinition, ResolutionError> {
    Ok(FunctionDefinition {
        name: raw_definition.name.clone(),
        arguments: extract_arguments(&raw_definition.name)?,
        body: raw_definition
            .body
            .into_iter()
            .map(resolve_statement)
            .collect::<Result<Vec<Statement>, ResolutionError>>()?,
    })
}
