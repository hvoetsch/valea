pub mod ast;
pub mod codegen;
pub mod diagnostics;
pub mod formatter;
pub mod json;
pub mod lexer;
pub mod parser;
pub mod typeck;

use ast::Program;
use diagnostics::Diagnostic;

pub fn parse_source(source: &str) -> Result<Program, Vec<Diagnostic>> {
    let tokens = lexer::lex(source)?;
    parser::parse(&tokens)
}

pub fn check_source(source: &str) -> Result<Program, Vec<Diagnostic>> {
    let program = parse_source(source)?;
    typeck::check(&program)?;
    Ok(program)
}
