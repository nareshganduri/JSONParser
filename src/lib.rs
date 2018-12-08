#![allow(dead_code)]
#![allow(unused_variables)]

pub mod json;
mod lexer;
mod parser;

use crate::parser::{ParseResult, Parser};

pub fn parse(input: &str) -> ParseResult {
    let mut parser = Parser::new(input);
    let result = parser.parse();

    result
}
